// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use jni::AttachGuard;
use ast_factory::*;
use verifier::Verifier;
use verifier::state;
use ast_utils::*;
use std::env;

pub struct VerificationContext<'a> {
    env: AttachGuard<'a>,
}

impl<'a> VerificationContext<'a> {
    pub fn new(env_guard: AttachGuard<'a>) -> Self {
        VerificationContext { env: env_guard }
    }

    pub fn new_ast_factory(&self) -> AstFactory {
        AstFactory::new(&self.env)
    }

    pub fn new_ast_utils(&self) -> AstUtils {
        AstUtils::new(&self.env)
    }

    pub fn new_verifier(&self) -> Verifier<state::Started> {
        self.new_verifier_with_args(vec![])
    }

    pub fn new_verifier_with_args(&self, args: Vec<&str>) -> Verifier<state::Started> {
        let z3_path = env::var("Z3_PATH").unwrap_or_else(|_| "/usr/bin/viper-z3".to_string());

        debug!("Using Z3 path: '{}'", &z3_path);

        let mut verifier_args = vec![];
        verifier_args.extend(args);
        verifier_args.extend(vec![
            "--z3Exe",
            &z3_path,
            "dummy-program.sil"
        ]);

        debug!("Verifier arguments: '{}'", verifier_args.iter().cloned().collect::<Vec<_>>().join(" "));

        Verifier::<state::Uninitialized>::new(&self.env)
            .parse_command_line(&verifier_args)
            .start()
    }
}
