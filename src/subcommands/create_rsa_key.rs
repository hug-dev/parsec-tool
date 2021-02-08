// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

//! Create a RSA key pair
//!
//! The key will be 2048 bits long. Used by default for asymmetric encryption with RSA PKCS#1 v1.5.

pub use crate::cli::ParsecToolApp;
use crate::error::ParsecToolError;
use crate::subcommands::common::ProviderOpts;
use crate::subcommands::ParsecToolSubcommand;
use parsec_client::core::interface::operations::psa_algorithm::AsymmetricEncryption;
use parsec_client::core::interface::operations::psa_generate_key;
use parsec_client::core::interface::operations::psa_key_attributes::{
    Attributes, Lifetime, Policy, Type, UsageFlags,
};
use parsec_client::core::interface::operations::{NativeOperation, NativeResult};
use parsec_client::core::operation_client::OperationClient;
use parsec_client::BasicClient;
use std::convert::TryFrom;
use structopt::StructOpt;

/// Create a RSA key pair.
#[derive(Debug, StructOpt)]
pub struct CreateRsaKey {
    #[structopt(short = "k", long = "key-name")]
    key_name: String,

    #[structopt(flatten)]
    provider_opts: ProviderOpts,
}

impl TryFrom<&CreateRsaKey> for NativeOperation {
    type Error = ParsecToolError;

    fn try_from(
        psa_generate_key_subcommand: &CreateRsaKey,
    ) -> Result<NativeOperation, Self::Error> {
        Ok(NativeOperation::PsaGenerateKey(
            psa_generate_key::Operation {
                key_name: psa_generate_key_subcommand.key_name.clone(),
                attributes: Attributes {
                    lifetime: Lifetime::Persistent,
                    key_type: Type::RsaKeyPair,
                    bits: 2048,
                    policy: Policy {
                        usage_flags: UsageFlags {
                            encrypt: true,
                            decrypt: true,
                            ..Default::default()
                        },
                        permitted_algorithms: AsymmetricEncryption::RsaPkcs1v15Crypt.into(),
                    },
                },
            },
        ))
    }
}

impl ParsecToolSubcommand<'_> for CreateRsaKey {
    /// Exports a key.
    fn run(
        &self,
        _matches: &ParsecToolApp,
        basic_client: BasicClient,
    ) -> Result<(), ParsecToolError> {
        info!("Creating RSA key...");

        let client = OperationClient::new();
        let native_result = client.process_operation(
            NativeOperation::try_from(self)?,
            self.provider_opts.provider()?,
            &basic_client.auth_data(),
        )?;

        match native_result {
            NativeResult::PsaGenerateKey(_) => (),
            _ => {
                return Err(ParsecToolError::UnexpectedNativeResult(native_result));
            }
        };

        success!("Key \"{}\" created.", self.key_name);
        Ok(())
    }
}