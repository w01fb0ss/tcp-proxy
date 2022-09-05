use tokio::{
    io,
    net::{TcpListener, TcpStream},
    select,
};

pub async fn proxy(client: &String, server: &String) -> io::Result<()> {
    let listener = TcpListener::bind(client).await?;
    loop {
        let (client_stream, _) = listener.accept().await?;
        let connect_stream = TcpStream::connect(server).await?;

        let (mut cread, mut cwrite) = client_stream.into_split();
        let (mut coread, mut cowrite) = connect_stream.into_split();

        let e2o = tokio::spawn(async move { io::copy(&mut cread, &mut cowrite).await });
        let o2e = tokio::spawn(async move { io::copy(&mut coread, &mut cwrite).await });

        select! {
                _ = e2o => println!("e2o done"),
                _ = o2e => println!("o2e done"),
        }
    }
}
