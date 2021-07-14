use std::prelude::v1::*;
use crate::sign::ecdsa::EcdsaKeyPair;
use crate::sign::ecdsa::KeyPair;
use num_bigint::BigInt;
use num_bigint::Sign::Plus;
use std::io::prelude::*;

use serde::{de, de::Deserializer, ser::Serializer, Deserialize, Serialize};

use super::PublicKey;
use crate::errors::*;
use crate::errors::Result;

///unsafe. 这里全是是为了按照超级链目前方式进行秘钥格式化
fn big_serialize<S>(x: &BigInt, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_str(x.to_str_radix(10).as_str())
}

fn big_deserialize<'de, D>(deserializer: D) -> std::result::Result<BigInt, D::Error>
where
    D: Deserializer<'de>,
{
    use std::str::FromStr;
    let s = String::deserialize(deserializer)?;
    let b = BigInt::from_str(s.as_str()).map_err(de::Error::custom)?;
    Ok(b)
}

#[derive(Serialize, Deserialize, Debug)]
struct ECDSAPrivateKey {
    #[serde(rename = "Curvname")]
    curve_name: String,
    #[serde(
        rename = "X",
        serialize_with = "big_serialize",
        deserialize_with = "big_deserialize"
    )]
    x: BigInt,
    #[serde(
        rename = "Y",
        serialize_with = "big_serialize",
        deserialize_with = "big_deserialize"
    )]
    y: BigInt,
    #[serde(
        rename = "D",
        serialize_with = "big_serialize",
        deserialize_with = "big_deserialize"
    )]
    d: BigInt,
}

impl ECDSAPrivateKey {
    fn from(sk: &EcdsaKeyPair) -> Self {
        let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
        let public_key = crate::sign::ecdsa::UnparsedPublicKey::new(alg, sk.public_key());
        let xy = public_key.xy();
        let x = BigInt::from_bytes_be(Plus, xy.0);
        let y = BigInt::from_bytes_be(Plus, xy.1);

        let seed = sk.seed_as_bytes();

        Self {
            curve_name: String::from("P-256"),
            x: x,
            y: y,
            d: BigInt::from_bytes_be(Plus, &seed),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct ECDSAPublicKey {
    #[serde(rename = "Curvname")]
    curve_name: String,
    #[serde(rename = "X", serialize_with = "big_serialize")]
    x: BigInt,
    #[serde(rename = "Y", serialize_with = "big_serialize")]
    y: BigInt,
}

impl ECDSAPublicKey {
    fn from<B: AsRef<[u8]>>(pk: &PublicKey<B>) -> Self {
        let xy = pk.xy();
        let x = BigInt::from_bytes_be(Plus, xy.0);
        let y = BigInt::from_bytes_be(Plus, xy.1);
        Self {
            curve_name: String::from("P-256"),
            x: x,
            y: y,
        }
    }
}

/// 将私钥转换成为json
///  格式例子： {"Curvname":"P-256","X":74695617477160058757747208220371236837474210247114418775262229497812962582435,"Y":51348715319124770392993866417088542497927816017012182211244120852620959209571,"D":29079635126530934056640915735344231956621504557963207107451663058887647996601}
///
pub fn get_ecdsa_private_key_json_format(k: &EcdsaKeyPair) -> Result<String> {
    let r = serde_json::to_string(&ECDSAPrivateKey::from(k))?;
    Ok(r)
}

//TODO useless. For compatibility to GO CRYPTO
// https://github.com/golang/go/issues/28154
fn del_quote(s: &str) -> String {
    //  "X":[0-9]+,  ->  "X": "[0-9]+",
    let matcher = r#""X":(")([0-9]+)(")"#;
    let re = regex::Regex::new(matcher).unwrap();
    let s = re.replace_all(s, |caps: &regex::Captures| format!("\"X\":{}", &caps[2]));

    let matcher = r#""Y":(")([0-9]+)(")"#;
    let re = regex::Regex::new(matcher).unwrap();
    let s = re.replace_all(&s, |caps: &regex::Captures| format!("\"Y\":{}", &caps[2]));

    let matcher = r#""D":(")([0-9]+)(")"#;
    let re = regex::Regex::new(matcher).unwrap();
    re.replace_all(&s, |caps: &regex::Captures| format!("\"D\":{}", &caps[2]))
        .to_string()
}

#[test]
fn test_del_quote() {
    let r = r#"{"Curvname":"P-256","X":"111614135018814739113902147296905176613869996836868032563930690178519466984733","Y":"13721631144572906234172791140317599374441573108268774470311606493984588103134","D":"101033109402914252566697805177509348009755604987980294429982156940657201471528"}"#;
    let d = del_quote(r);
    let rd = r#"{"Curvname":"P-256","X":111614135018814739113902147296905176613869996836868032563930690178519466984733,"Y":13721631144572906234172791140317599374441573108268774470311606493984588103134,"D":101033109402914252566697805177509348009755604987980294429982156940657201471528}"#;
    assert_eq!(rd, d.as_str());

    let r = r#"{"Curvname":"P-256","X":"111614135018814739113902147296905176613869996836868032563930690178519466984733","Y":"13721631144572906234172791140317599374441573108268774470311606493984588103134"}"#;
    let d = del_quote(r);
    let rd = r#"{"Curvname":"P-256","X":111614135018814739113902147296905176613869996836868032563930690178519466984733,"Y":13721631144572906234172791140317599374441573108268774470311606493984588103134}"#;
    assert_eq!(rd, d.as_str());
}

pub fn get_ecdsa_public_key_json_format_in_go(k: &EcdsaKeyPair) -> Result<String> {
    Ok(del_quote(get_ecdsa_public_key_json_format(k)?.as_str()))
}

pub fn get_ecdsa_public_key_json_format(k: &EcdsaKeyPair) -> Result<String> {
    let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
    let public_key = crate::sign::ecdsa::UnparsedPublicKey::new(alg, k.public_key());
    let r = serde_json::to_string(&ECDSAPublicKey::from(&public_key))?;
    Ok(r)
}

pub fn get_ecdsa_private_key_from_json(key_str: &str) -> Result<EcdsaKeyPair> {
    //判断曲线 TODO
    let acc: ECDSAPrivateKey = serde_json::from_str(key_str)?;
    let seed_bytes = acc.d.to_bytes_be();
    let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING;
    let seed = untrusted::Input::from(&seed_bytes.1);
    let private_key = crate::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(alg, seed)?;
    Ok(private_key)
}

pub fn get_ecdsa_private_key_from_file(filename: &str) -> Result<EcdsaKeyPair> {
    let mut f = std::fs::File::open(std::path::PathBuf::from(filename))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    get_ecdsa_private_key_from_json(contents.as_str())
}

pub fn get_ecdsa_public_key_from_json(key_str: &str) -> Result<Vec<u8>> {
    let acc: ECDSAPublicKey = serde_json::from_str(key_str)?;
    let mut seed = vec![4u8; 1];
    seed.extend_from_slice(&acc.x.to_bytes_be().1);
    seed.extend_from_slice(&acc.y.to_bytes_be().1);
    //let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
    //let public_key = crate::sign::ecdsa::UnparsedPublicKey::new(alg, seed);
    //Ok(public_key.as_ref())
    Ok(seed)
}

pub fn get_ecdsa_public_key_from_file(filename: &str) -> Result<Vec<u8>> {
    let mut f = std::fs::File::open(std::path::PathBuf::from(filename))?;
    let mut contents = String::new();
    f.read_to_string(&mut contents)?;
    get_ecdsa_public_key_from_json(contents.as_str())
}

pub fn get_ecdsa_public_key_json_format_from_public_key<'a, B: AsRef<[u8]>>(
    pk: &PublicKey<B>,
) -> Result<String> {
    let r = serde_json::to_string(&ECDSAPublicKey::from(pk))?;
    Ok(r)
}

#[test]
fn test_json_public() {
    let key_slice = hex::decode(
        "04a664e9bbf6d03e4b75758f7ee3732a0a8eff9e76a0edc9a14ca584b966493664d0d8b7871c5b33bdee9f0e154d7eb948356229e7694cb04a785520952dae1438",
    )
    .unwrap();

    let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1;
    let public_key = crate::sign::ecdsa::UnparsedPublicKey::new(alg, &key_slice);
    let msg = String::from("hello world");
    let sig = hex::decode("3046022100873aad44cea8badf28c8f6b4509763e875a21805daf971bffc3a9bd27288a30b022100899216a47e3f071ede3d697bb172b94a9240d0c8cc6a5754a68edc00e1752873").unwrap();
    let res = public_key.verify(&msg.as_bytes(), &sig);
    println!("{:?}", res);

    let res = get_ecdsa_public_key_json_format_from_public_key(&public_key).unwrap();
    println!("json: {:?}", res);
}

#[test]
pub fn test_json_private() {
    use std::str::FromStr;
    let d = "29079635126530934056640915735344231956621504557963207107451663058887647996601";
    let seed_bytes = num_bigint::BigInt::from_str(&d).unwrap().to_bytes_be();
    let alg = &crate::sign::ecdsa::ECDSA_P256_SHA256_ASN1_SIGNING;
    let seed = untrusted::Input::from(&seed_bytes.1);
    let private_key = crate::sign::ecdsa::EcdsaKeyPair::from_seed_unchecked(alg, seed);
    assert_eq!(private_key.is_ok(), true);
    let private_key = private_key.unwrap();
    let res = get_ecdsa_private_key_json_format(&private_key);
    assert_eq!(res.is_ok(), true);
    println!("json: {:?}", res);
    let sk2 = get_ecdsa_private_key_from_json(res.unwrap().as_str());
    assert_eq!(sk2.is_ok(), true);
    assert_eq!(sk2.unwrap().seed_as_bytes(), private_key.seed_as_bytes());
}
