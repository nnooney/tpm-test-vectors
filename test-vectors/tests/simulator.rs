use anyhow::Context;
use rstest::rstest;
use std::process::{Child, Command};
use tpm2_test_vectors::TpmTestVector;

use tpm2_rs_client::connection::{Connection, SimulatorPlatformSignal, TcpConnection};

#[rstest]
fn test_vector(
    #[files("src/vectors/*.ron")]
    #[mode = str]
    input: &str,
) -> anyhow::Result<()> {
    let test_case: TpmTestVector = ron::from_str(input)?;

    let _simulator = TpmSimulator::new()?;
    let mut conn = connect_to_simulator()?;

    for command in test_case.test_sequence {
        let mut resp = vec![0; command.response.len()];
        conn.transact(&command.input, &mut resp)?;
        assert_eq!(
            resp, command.response,
            "step \"{}\" response mismatch",
            command.step
        );
    }

    Ok(())
}

/// Environment variable used to connect to the TPM simulator over TCP.
const ENV_VAR_SIMULATOR_IP: &str = "SIMULATOR_IP";

/// Default IP address of the TPM simulator program. This value assumes the
/// simulator is running on the same location as the test.
const DEFAULT_SIMULATOR_IP: &str = "127.0.0.1";

/// Get the IP address to connect to the TPM simulator. Set the environment
/// variable at the command line to specify a different IP address, e.g.
///
/// ```shell
/// SIMULATOR_IP="192.168.1.1" cargo test
/// ```
fn get_simulator_ip() -> String {
    std::env::var(ENV_VAR_SIMULATOR_IP).unwrap_or(DEFAULT_SIMULATOR_IP.to_string())
}

/// Environment variable used to override the TPM simulator program.
const ENV_VAR_SIMULATOR_PROGRAM: &str = "SIMULATOR_BIN";

/// Default location of the TPM simulator program. This value assumes we're
/// running in a docker container built by the TPM-provided Dockerfile.
const DEFAULT_SIMULATOR_PROGRAM: &str = "/tpm2-simulator";

/// Get the program to run to launch the TPM simulator. Set the environment
/// variable at the command line to specify a different program to run, e.g.
///
/// ```shell
/// SIMULATOR_BIN="/my/custom/simulator --start" cargo test
/// ```
fn get_simulator_path() -> String {
    std::env::var(ENV_VAR_SIMULATOR_PROGRAM).unwrap_or(DEFAULT_SIMULATOR_PROGRAM.to_string())
}

/// Structure to manage the subprocess used to spawn the TPM simulator.
pub struct TpmSimulator(Child);

impl TpmSimulator {
    fn new() -> anyhow::Result<TpmSimulator> {
        let simulator_bin = get_simulator_path();
        Ok(TpmSimulator(
            Command::new(&simulator_bin)
                .current_dir("/")
                .spawn()
                .context(format!("failed to start TPM simulator \"{simulator_bin}\""))?,
        ))
    }
}

impl Drop for TpmSimulator {
    fn drop(&mut self) {
        if let Err(x) = self.0.kill() {
            println!("Failed to stop simulator: {x}");
        }
    }
}

/// Function to connect to the TPM simulator, with retry logic.
fn connect_to_simulator() -> anyhow::Result<TcpConnection> {
    let mut attempts = 0;

    let mut conn = loop {
        attempts += 1;
        match TcpConnection::new_default(&get_simulator_ip()) {
            Ok(conn) => break conn,
            Err(err) => {
                if attempts > 3 {
                    return Err(err.into());
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        }
    };

    // Issue the sequence of signals to initialize the TPM simulator.
    conn.platform_signal(SimulatorPlatformSignal::NvOff)?;
    conn.platform_signal(SimulatorPlatformSignal::PowerOff)?;
    conn.platform_signal(SimulatorPlatformSignal::PowerOn)?;
    conn.platform_signal(SimulatorPlatformSignal::NvOn)?;

    Ok(conn)
}
