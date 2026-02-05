use rstest::rstest;
use tpm2_rs_client::connection::{Connection, TcpConnection, TcpSimulator};
use tpm2_test_vectors::{Harness, HarnessError};

mod common;

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
/// SIMULATOR_BIN="/my/custom/simulator" cargo test
/// ```
fn get_simulator_path() -> String {
    std::env::var(ENV_VAR_SIMULATOR_PROGRAM).unwrap_or(DEFAULT_SIMULATOR_PROGRAM.to_string())
}

/// Environment variable used to override the arguments to the TPM simulator.
const ENV_VAR_SIMULATOR_ARGS: &str = "SIMULATOR_ARGS";

/// Default arguments to pass to the TPM simulator.
const DEFAULT_SIMULATOR_ARGS: &str = "";

/// Get the arguments to pass to the TPM simulator. Set the environment
/// variable at the command line to specify different arguments, e.g.
///
/// ```shell
/// SIMULATOR_ARGS="--custom-arg" cargo test
/// ```
fn get_simulator_args() -> Vec<String> {
    std::env::var(ENV_VAR_SIMULATOR_ARGS)
        .unwrap_or(DEFAULT_SIMULATOR_ARGS.to_string())
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

/// Structure which implements the [`harness::Harness`] trait for interacting
/// with the TPM simulator.
pub struct TpmSimulatorHarness {
    simulator: TcpSimulator,
}

type TcpConnectionError = <TcpConnection as Connection>::Error;

impl TpmSimulatorHarness {
    pub fn new() -> anyhow::Result<TpmSimulatorHarness> {
        let simulator = TcpSimulator::new(
            get_simulator_path(),
            get_simulator_args().as_slice(),
            &get_simulator_ip(),
        )?;

        Ok(TpmSimulatorHarness { simulator })
    }

    pub fn init_tpm(&mut self) -> Result<(), HarnessError<TcpConnectionError>> {
        self.simulator.connection_mut().reinit()?;
        Ok(())
    }
}

impl Harness<TcpConnectionError> for TpmSimulatorHarness {
    fn transact(
        &mut self,
        cmd: &[u8],
        rsp: &mut [u8],
    ) -> Result<(), HarnessError<TcpConnectionError>> {
        self.simulator.connection_mut().transact(cmd, rsp)?;
        Ok(())
    }

    fn set_failure_mode(&mut self) -> Result<(), HarnessError<TcpConnectionError>> {
        self.simulator.connection_mut().test_failure_mode()?;
        Ok(())
    }
}

#[rstest]
fn simulator(
    #[files("src/vectors/*.ron")]
    #[mode = str]
    input: &str,
) -> anyhow::Result<()> {
    let mut harness = TpmSimulatorHarness::new()?;
    harness.init_tpm()?;

    common::run_test_vector(input, &mut harness)
}
