mod bash;
mod fish;
mod nu;
mod pwsh;
mod zsh;

use crate::model::AliasConfig;

pub(crate) fn init(config: AliasConfig, args: Vec<String>) {
    let shell = args.first().unwrap().as_ref();
    match shell {
        "powershell" => {
            pwsh::init(config);
        }
        "zsh" => {
            zsh::init(config);
        }
        "bash" => {
            bash::init(config);
        }
        "fish" => {
            fish::init(config);
        }
        "nu" => {
            nu::init(config);
        }
        _ => unreachable!("Unsupported shell"),
    }
}
