use std::io;
use std::io::prelude::*;
use std::str;
use std::fs::{File, DirBuilder};
use std::path::Path;
use std::boxed::Box;
use termios::{Termios, TCSANOW, ECHO, ICANON, tcsetattr};
use failure::{Error, SyncFailure, ResultExt};

use acme_client::openssl;
use acme_client::openssl::pkey::PKey;
use acme_client::openssl::rsa::Rsa;
use acme_client::LETSENCRYPT_DIRECTORY_URL;
use acme_client::Directory;
use acme_client::Account;
use acme_client::SignedCertificate;

use super::DockerContainerDetails;
use super::OwlPlugin;

const FFDHE2048:&[u8] = b"-----BEGIN DH PARAMETERS-----
MIIBCAKCAQEA//////////+t+FRYortKmq/cViAnPTzx2LnFg84tNpWp4TZBFGQz
+8yTnc4kmz75fS/jY2MMddj2gbICrsRhetPfHtXV/WVhJDP1H18GbtCFY2VVPe0a
87VXE15/V8k1mE8McODmi3fipona8+/och3xWKE2rec1MKzKT0g6eXq8CrGCsyT7
YdEIqUuyyOP7uWrat2DX9GgdT0Kj3jlN9K5W7edjcrsZCwenyO4KbXCeAvzhzffi
7MA0BM0oNC9hkXL+nOmFg/+OTxIy7vKBg8P+OxtMb61zO7X8vC7CIAXFjvGDfRaD
ssbzSibBsu/6iGtCOGEoXJf//////////wIBAg==
-----END DH PARAMETERS-----";

const FFDHE3072:&[u8] = b"-----BEGIN DH PARAMETERS-----
MIIBiAKCAYEA//////////+t+FRYortKmq/cViAnPTzx2LnFg84tNpWp4TZBFGQz
+8yTnc4kmz75fS/jY2MMddj2gbICrsRhetPfHtXV/WVhJDP1H18GbtCFY2VVPe0a
87VXE15/V8k1mE8McODmi3fipona8+/och3xWKE2rec1MKzKT0g6eXq8CrGCsyT7
YdEIqUuyyOP7uWrat2DX9GgdT0Kj3jlN9K5W7edjcrsZCwenyO4KbXCeAvzhzffi
7MA0BM0oNC9hkXL+nOmFg/+OTxIy7vKBg8P+OxtMb61zO7X8vC7CIAXFjvGDfRaD
ssbzSibBsu/6iGtCOGEfz9zeNVs7ZRkDW7w09N75nAI4YbRvydbmyQd62R0mkff3
7lmMsPrBhtkcrv4TCYUTknC0EwyTvEN5RPT9RFLi103TZPLiHnH1S/9croKrnJ32
nuhtK8UiNjoNq8Uhl5sN6todv5pC1cRITgq80Gv6U93vPBsg7j/VnXwl5B0rZsYu
N///////////AgEC
-----END DH PARAMETERS-----";

const FFDHE4096:&[u8] = b"-----BEGIN DH PARAMETERS-----
MIICCAKCAgEA//////////+t+FRYortKmq/cViAnPTzx2LnFg84tNpWp4TZBFGQz
+8yTnc4kmz75fS/jY2MMddj2gbICrsRhetPfHtXV/WVhJDP1H18GbtCFY2VVPe0a
87VXE15/V8k1mE8McODmi3fipona8+/och3xWKE2rec1MKzKT0g6eXq8CrGCsyT7
YdEIqUuyyOP7uWrat2DX9GgdT0Kj3jlN9K5W7edjcrsZCwenyO4KbXCeAvzhzffi
7MA0BM0oNC9hkXL+nOmFg/+OTxIy7vKBg8P+OxtMb61zO7X8vC7CIAXFjvGDfRaD
ssbzSibBsu/6iGtCOGEfz9zeNVs7ZRkDW7w09N75nAI4YbRvydbmyQd62R0mkff3
7lmMsPrBhtkcrv4TCYUTknC0EwyTvEN5RPT9RFLi103TZPLiHnH1S/9croKrnJ32
nuhtK8UiNjoNq8Uhl5sN6todv5pC1cRITgq80Gv6U93vPBsg7j/VnXwl5B0rZp4e
8W5vUsMWTfT7eTDp5OWIV7asfV9C1p9tGHdjzx1VA0AEh/VbpX4xzHpxNciG77Qx
iu1qHgEtnmgyqQdgCpGBMMRtx3j5ca0AOAkpmaMzy4t6Gh25PXFAADwqTs6p+Y0K
zAqCkc3OyX3Pjsm1Wn+IpGtNtahR9EGC4caKAH5eZV9q//////////8CAQI=
-----END DH PARAMETERS-----";

#[derive(Debug)]
pub struct Acme {
    acme_url: &'static str,
    default_dest: String,
    default_webroot: String,
    default_challenge: String,
    default_hostname: String,
    default_mail: String,
    default_keysize: u32
}

// pub enum AcmeError {
//     HTTP_CHALLENGE_NOT_FOUND,
//     DNS_CHALLENGE_NOT_FOUND,
// }

fn ask_to_continue() {
    // Save termcaps
    let stdin = 0;  // couldn't get std::os::unix::io::FromRawFd to work
                    // on /dev/stdin or /dev/tty
    let termios = Termios::from_fd(stdin).unwrap();

    // Change termcaps
    let mut new_termios = termios;           // make a mutable copy of termios
    new_termios.c_lflag &= !(ICANON | ECHO); // no echo and canonical mode
    tcsetattr(stdin, TCSANOW, &new_termios).unwrap();

    // Wait enter user input key
    println!("Press enter to continue");
    let mut input = [0;1];  // read exactly one byte
    let mut wait = true;
    while wait {
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_exact(&mut input).unwrap();
        let str = str::from_utf8(&input).unwrap();
        if str == "\n" {
            wait = false;
        }
    }

    // Restore termcaps
    tcsetattr(stdin, TCSANOW, &termios).unwrap();
}

impl Acme {

    pub fn new(debug: bool) -> Box<OwlPlugin + Send + Sync> {
        let default_dest = String::from("./certs");
        let default_webroot = String::from("./www");
        let default_challenge = String::from("dns");
        let default_hostname = String::from("localhost");
        let default_mail = String::from("");
        let acme_url = if debug {
            "https://acme-staging.api.letsencrypt.org/directory"
        } else {
            LETSENCRYPT_DIRECTORY_URL
        };
        Box::new(Acme {
            acme_url,
            default_dest,
            default_webroot,
            default_challenge,
            default_hostname,
            default_mail,
            default_keysize: 0x1000  // 4096
        })
    }

    pub fn create_account(&self, mail: &str) -> Result<Account, Error> {

        // 1. Create AccountRegistration object helper
        let directory = Directory::from_url(self.acme_url).map_err(SyncFailure::new)?;
        let mut account_registration = directory
            .account_registration()
            .email(mail);

        // 2. Create or read private account key if already exist
        let pkey_path = Path::new("./certs").join("account_pkey");
        let pkey_path_str = pkey_path.to_str().expect("No ./certs directory");
        if pkey_path.is_file() {
            account_registration = account_registration.pkey_from_file(pkey_path_str).map_err(SyncFailure::new)?;
        } else {
            warn!("PKEY '{}' not found, we will generate one", pkey_path_str);
        }

        // 3. Register account
        let account = account_registration
            .register()
            .map_err(SyncFailure::new)?;

        // 4. Save private account key
        if !pkey_path.is_file() {
            account.save_private_key(pkey_path_str).map_err(SyncFailure::new).context(format!("File {:?}", pkey_path))?;
        }

        Ok(account)
    }

    fn dns_validation(account: &Account, hostname: &str) -> Result<(), Error> {

        // 1. Create Authorization object helper
        let authorization = account.authorization(hostname).map_err(SyncFailure::new)?;

        // 2. Get dns challenge
        let dns_challenge = authorization.get_dns_challenge().unwrap();  // TODO, improve error handling
            // .ok_or_else(|| AcmeError::HTTP_CHALLENGE_NOT_FOUND)?;

        // 3. Print signature and wait user
        let signature = dns_challenge.signature().map_err(SyncFailure::new)?;
        println!("Signature : {:?}", signature);
        println!("Signature must be saved as a TXT record for _acme_challenge.{}", hostname);
        ask_to_continue();
        println!("Continue...");

        // 4. Run dns challenge
        dns_challenge.validate().map_err(SyncFailure::new)?;

        Ok(())
    }

    fn http_validation(account: &Account, hostname: &str, webroot: &str) -> Result<(), Error> {
        // 1. Create Authorization object helper
        let authorization = account.authorization(hostname).map_err(SyncFailure::new)?;

        // 2. Get http challenge
        let http_challenge = authorization.get_http_challenge()  // TODO, improve error handling
            .ok_or("HTTP challenge not found")
            .unwrap();

        // 3. Save signatue in '{path}/.well-known/acme-challenge/' directory
        http_challenge.save_key_authorization(webroot).map_err(SyncFailure::new)?;

        // 4. Run http challenge
        http_challenge.validate().map_err(SyncFailure::new)?;

        Ok(())
    }

    /// Generates a new PKey
    fn gen_key(key_size: u32) -> PKey<openssl::pkey::Private> {
        let rsa = Rsa::generate(key_size).unwrap();
        PKey::from_rsa(rsa).unwrap()
    }

    fn sign_certificate(account: &Account, domains: &[&str], keysize: u32) -> Result<SignedCertificate, Error>  {

        // 1. Create CertificateSigner object helper
        let certificate_signer = account.certificate_signer(domains);

        // 2. Use custom private key
        let pkey = Acme::gen_key(keysize);

        // 3. Sign certificate
        let cert = certificate_signer
            .pkey(pkey)
            .sign_certificate()
            .map_err(SyncFailure::new)?;

        Ok(cert)
    }
}


impl OwlPlugin for Acme {

    fn get_name(&self) -> &'static str {
        const NAME: &str = "Acme";
        NAME
    }

    fn should_process(&self, details: &DockerContainerDetails) -> bool {
        details.config.labels.as_ref().map_or(false, |labels| labels.contains_key("letsencrypt-enable"))
    }

    fn process(&self, details: &DockerContainerDetails) -> Result<(), Error> {
        let label = &details.config.labels.as_ref().unwrap();

        let dest = label.get("letsencrypt-dest").unwrap_or(&self.default_dest);
        let webroot = label.get("letsencrypt-webroot").unwrap_or(&self.default_webroot);
        let challenge = label.get("letsencrypt-challenge").unwrap_or(&self.default_challenge);
        let hostname = label.get("letsencrypt-hostname").unwrap_or(&self.default_hostname);
        let mail = label.get("letsencrypt-mail").unwrap_or(&self.default_mail);
        let keysize = label
            .get("letsencrypt-keysize")
            .map_or(self.default_keysize, |s| s.parse::<u32>().unwrap_or(self.default_keysize));
        let dh_group = label.get("letsencrypt-dh-group");
        let dh_str_path = format!("{}/dhparam.pem", dest);
        let dh_path = Path::new(&dh_str_path);


        // 0. Create directories
        DirBuilder::new().recursive(true).create(dest).context(format!("Directory {:?}", dh_path))?;
        DirBuilder::new().recursive(true).create(webroot).context(format!("Directory {:?}", dh_path))?;

        // 1. Create ACME account
        let account = self.create_account(mail)?;

        // 2. Verify domain name
        match challenge.as_str() {
            "dns"  => Acme::dns_validation(&account, hostname)?,
            "http" => Acme::http_validation(&account, hostname, webroot)?,
            _ => warn!("No challenge found")
        }

        // 3. Create certificate
        let domains = vec![hostname.as_str()];
        let certificate = Acme::sign_certificate(&account, &domains, keysize)?;

        // 4. Save generated files
        certificate.save_signed_certificate(format!("{}/{}.pem", dest, domains[0])).map_err(SyncFailure::new)?;
        certificate.save_private_key(format!("{}/{}.key", dest, domains[0])).map_err(SyncFailure::new)?;

        // 5. Create dh file
        if dh_group.is_some() && !dh_path.is_file() {
            let dh_buffer = match dh_group.unwrap().as_str() {
                "ffdhe2048" => Some(FFDHE2048),
                "ffdhe3072" => Some(FFDHE3072),
                "ffdhe4096" => Some(FFDHE4096),
                _ => None
            }.unwrap();
            let mut buffer = File::create(dh_path).context(format!("File {:?}", dh_path))?;
            buffer.write_all(dh_buffer)?;
        }

        Ok(())
    }
}
