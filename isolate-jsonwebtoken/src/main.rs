use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use models::TokenClaims;
use std::fs;
use std::time::Instant;

mod models {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct TokenClaims {
        pub sub: String,
        pub iss: String,
        pub aud: String,
        pub iat: usize,
        pub exp: usize,
    }
}

fn main() {
    let start = Instant::now();

    let n1 = start.elapsed();
    let private_pem_file_content = fs::read_to_string("privatekey-authx.pkcs8")
        .expect("Should have been able to read the file");
    // println!("{}", private_pem_file_content);
    // let key_pair =
    //     RS256KeyPair::from_pem(&private_pem_file_content).expect("Could not read private key");
    // let n11 = start.elapsed();
    // println!("Elapsed load keypair {}", (n11 - n1).as_millis());

    let mut i = 0;
    while i < 10 {
        let n2 = start.elapsed();
        // let id_claims = IdClaims {
        //     username: "NPA-PlatformManagement".to_string(),
        //     email: "usr001..".to_string(),
        // };
        // let claims =
        //     Claims::with_custom_claims(id_claims, coarsetime::Duration::from_secs(60 * 60 * 2))
        //         .with_issuer("https://authx.xlinq.io")
        //         .with_audience("scf.xlinq.io");
        // let token = key_pair.sign(claims).expect("Could not sign claims");
        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(60)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: "NPA-PlatformManagement".to_string(),
            iss: "https://authx.xlinq.io".to_string(),
            aud: "scf.xlinq.io".to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::new(Algorithm::RS256),
            &claims,
            &EncodingKey::from_rsa_pem(include_bytes!("../privatekey-authx.pkcs8")).unwrap(),
        ).unwrap();
        let n3 = start.elapsed();

        println!("Elapsed sign {} token {}", (n3 - n2).as_millis(), token);
        i = i + 1;
    }
}
