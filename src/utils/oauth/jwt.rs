
struct JwtDecoder {
    issuer: String, 
    secret: Option<String>,
    audience: String,
    validate: bool
}



impl JwtDecoder {

}