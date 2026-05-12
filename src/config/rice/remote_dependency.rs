use crate::package_manager::dependency::Dependency;
use crate::system::web::get_file_name_from_url;
use crate::ErrorMessages;

pub fn install_remote_dependencies(
    dependencies: &Vec<Dependency>,
) -> color_eyre::Result<ErrorMessages> {
    let dependencies = dependencies
        .iter()
        .filter(|dependency| matches!(dependency, Dependency::RemoteInstaller { .. }))
        .map(|dependency| match dependency {
            Dependency::RemoteInstaller {
                url,
                sha256,
                args,
                check_command,
            } => (url, sha256, args, check_command),
            _ => unreachable!(),
        })
        .collect::<Vec<_>>();

    // run check command before continuing

    let mut error_messages = vec![];

    for (url, sha256, args, check_command) in dependencies {
        let Some(file_name) = get_file_name_from_url(url) else {
            error_messages.push(format!("Could not get file name from url {}, abort.", url));
            continue;
        };

        // stream file into temp file
        // check hash
        // install with provided args
    }

    Ok(error_messages)
}
