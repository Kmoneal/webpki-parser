extern crate webpki;
extern crate untrusted;


fn main() {
    //let cert_file = OpenOptions::new()
    //    .read(true)
    //    open("private/cert.pem")
    //    .expect("Openning file");

    //let mut reader = BufReader::new(cert_file);
    //let cert = pemfile::certs(&mut reader).expect("");

    //let input = untrusted::Input::from(cert[0].as_ref());
    let ee = include_bytes!("private/ee.der");
    let ee_input = untrusted::Input::from(ee);

    let cert = webpki::EndEntityCert::from(ee_input).expect("end entity");
    println!("Validity: {:#?}", cert.get_validity().unwrap());
    println!("Issuer: {:?}", cert.get_issuer());
    println!("Signing Alg: {:#X?}", cert.get_signing_alg());
    println!("Subject: {:?}", cert.get_subject());
    println!("Alt Name(s): {:?}", cert.get_subject_alt_name());
}
