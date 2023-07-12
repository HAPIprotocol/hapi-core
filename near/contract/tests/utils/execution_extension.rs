use std::ops::Div;

use near_sdk::serde::de::DeserializeOwned;
use workspaces::result::{
    ExecutionFailure, ExecutionFinalResult, ExecutionSuccess, ViewResultDetails,
};

use crate::{ONE_TGAS, SHOW_DEFAULT_OUTPUT, SHOW_LOGS};

/* ************** */
/* VIEW EXTENSION */
/* ************** */
pub trait ViewResultDetailsExtension {
    fn get_result<T: DeserializeOwned>(self, action: &str) -> T;
}

pub trait ViewResultDetailsHelper {
    fn show_logs(&self);
}

impl ViewResultDetailsExtension for workspaces::Result<ViewResultDetails> {
    fn get_result<T: DeserializeOwned>(self, action: &str) -> T {
        match self {
            Ok(this) => {
                if SHOW_DEFAULT_OUTPUT {
                    println!("=== VIEW '{action}' TRANSACTION ===");
                }

                this.show_logs();

                this.json::<T>().expect("Can't deserialize view result")
            }
            Err(this) => {
                println!("=== VIEW '{action}' TRANSACTION ===");
                panic!(
                    "---> Unhandled exception has occurred. Error msg\n{:#?}",
                    this
                );
            }
        }
    }
}

impl ViewResultDetailsHelper for ViewResultDetails {
    fn show_logs(&self) {
        if SHOW_LOGS && !self.logs.is_empty() {
            println!("{:#?}", self.logs);
        }
    }
}

/* ************** */
/* CALL EXTENSION */
/* ************** */
pub trait CallExecutionDetailsExtension {
    fn assert_failure(self, action: &str, expect: &str);
    fn assert_success(self, action: &str) -> ExecutionSuccess;
}

pub trait CallExecutionDetailsHelper {
    fn show_logs(&self);
    fn show_outcomes(&self, show: bool);
    fn get_result<T: DeserializeOwned>(self) -> T;
}

impl CallExecutionDetailsExtension for workspaces::Result<ExecutionFinalResult> {
    fn assert_failure(self, action: &str, expect: &str) {
        match self {
            Ok(this) => match this.into_result() {
                Ok(success) => {
                    let mut has_error = false;

                    for receipt in success.receipt_failures().into_iter() {
                        match receipt.clone().into_result() {
                            Err(error) => {
                                if error
                                    .into_inner()
                                    .expect("ERR: Get inner error data")
                                    .to_string()
                                    .contains(expect)
                                {
                                    has_error = true;
                                    break;
                                }
                            }
                            _ => unimplemented!(),
                        }
                    }

                    if SHOW_DEFAULT_OUTPUT || !has_error {
                        println!("=== BEGIN '{action}' TRANSACTION ===");
                    }

                    success.show_logs();
                    success.show_outcomes(!has_error);

                    assert!(
                        has_error,
                        "---> Expect error: {expect}. Found\n{:#?}",
                        success.failures()
                    );

                    if SHOW_DEFAULT_OUTPUT {
                        println!("\n---> Got error as expected: {expect}\n");
                        println!("=== END '{action}' TRANSACTION ===\n");
                    }
                }
                Err(error) => {
                    let has_error = error.to_string().contains(expect);

                    if SHOW_DEFAULT_OUTPUT || !has_error {
                        println!("=== BEGIN '{action}' TRANSACTION ===");
                    }

                    error.show_logs();
                    error.show_outcomes(!has_error);

                    assert!(
                        has_error,
                        "---> Expect error: {expect}. Found\n{:#?}",
                        error.failures()
                    );

                    if SHOW_DEFAULT_OUTPUT {
                        println!("\n---> Got error as expected: {expect}\n");
                        println!("=== END '{action}' TRANSACTION ===\n");
                    }
                }
            },
            Err(this) => {
                let has_error = this.to_string().contains(expect);

                if SHOW_DEFAULT_OUTPUT || !has_error {
                    println!("=== BEGIN '{action}' TRANSACTION ===");
                }

                assert!(
                    has_error,
                    "---> Expect error: {expect}, found:\n{:#?}",
                    this
                );

                if SHOW_DEFAULT_OUTPUT {
                    println!("\n---> Got error as expected: {expect}\n");
                    println!("=== END '{action}' TRANSACTION ===\n");
                }
            }
        }
    }

    fn assert_success(self, action: &str) -> ExecutionSuccess {
        match self {
            Ok(this) => match this.into_result() {
                Ok(success) => {
                    let has_error = !success.receipt_failures().is_empty();
                    if SHOW_DEFAULT_OUTPUT || has_error {
                        println!("=== BEGIN '{action}' TRANSACTION ===");
                    }

                    success.show_logs();
                    success.show_outcomes(has_error);

                    assert!(
                        !has_error,
                        "---> Expect '{}' success. {:#?}",
                        action,
                        success.receipt_failures()
                    );

                    if SHOW_DEFAULT_OUTPUT {
                        println!("=== END '{action}' TRANSACTION ===\n");
                    }

                    success
                }
                Err(failure) => {
                    println!("=== BEGIN '{action}' TRANSACTION ===");
                    panic!(
                        "---> Inner unhandled exception has occurred. Error msg\n{:#?}",
                        failure.failures()
                    );
                }
            },
            Err(this) => {
                println!("=== BEGIN '{action}' TRANSACTION ===");
                panic!(
                    "---> Unhandled exception has occurred. Error msg\n{:#?}",
                    this
                );
            }
        }
    }
}

impl CallExecutionDetailsHelper for ExecutionSuccess {
    fn get_result<T: DeserializeOwned>(self) -> T {
        self.json().expect("Can't unwrap result")
    }

    fn show_logs(&self) {
        if SHOW_LOGS && !self.logs().is_empty() {
            println!("{:#?}", self.logs());
        }
    }

    fn show_outcomes(&self, show: bool) {
        if SHOW_DEFAULT_OUTPUT || show {
            println!("=== OUTCOMES {:#?}", self.receipt_outcomes());
            println!(
                "=== GAS BURNT {} TGas ===",
                self.total_gas_burnt.div(10_u64.pow(12_u32))
            ); // Convert to TGas
        }
    }
}

impl CallExecutionDetailsHelper for ExecutionFailure {
    fn get_result<T: DeserializeOwned>(self) -> T {
        unimplemented!()
    }

    fn show_logs(&self) {
        if SHOW_LOGS && !self.logs().is_empty() {
            println!("{:#?}", self.logs());
        }
    }

    fn show_outcomes(&self, show: bool) {
        if SHOW_DEFAULT_OUTPUT || show {
            println!("=== OUTCOMES {:#?}", self.receipt_outcomes());
            println!(
                "=== GAS BURNT {} TGas ===",
                self.total_gas_burnt.div(ONE_TGAS)
            ); // Convert to TGas
        }
    }
}
