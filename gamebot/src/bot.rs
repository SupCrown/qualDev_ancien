use std::net::TcpStream;



#[cfg!(test)]
mod tests{
    use std::net::TcpStream;

    #[test]
    fn creation_bot(){
        let connection = TcpStream::connect("172.17.0.1:6969");
        let bot = Bot{
            &connection,
            name: "fabrice",
            motR: 1.0,
            motL: 1.0,
        };
        assert_eq!(bot.connection, connection);
        assert_eq!(bot.name, "fabrice");
        assert_eq!(bot.motR, 1.0);
        assert_eq!(bot.motL, 1.0);
;
    }
}