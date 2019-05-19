extern crate reqwest;
use http::{Request, Response};
use std::collections::HashMap;

pub fn getGiphy(text: String) -> Result<(), Box<std::error::Error>> {
    let uri = format!("http://api.giphy.com/v1/gifs/search?api_key=Y99QRjzrSNP0HucWPPtXMnNJh3ERdf1o&q={}", text);
    let body = reqwest::get("http://api.giphy.com/v1/gifs/search?api_key=Y99QRjzrSNP0HucWPPtXMnNJh3ERdf1o&q=rust")?
    .text()?;

    println!("{:#?}", body);
    Ok(())
}


// pub fn getGiphy(text: String) {
//     // let mut request = Request::builder();
//     // request.uri("http://api.giphy.com/v1/gifs/search?api_key=Y99QRjzrSNP0HucWPPtXMnNJh3ERdf1o&q={}", text);
//     // let response = send(request.body(()).unwrap());
//     // println!("response{}", response);



//     // let request = Request::builder()
//     //   .uri(uri)
//     //   .body(())
//     //   .unwrap();

//     // println!("request{:#?}", request);




// //   const url = `http://api.giphy.com/v1/gifs/search?api_key=Y99QRjzrSNP0HucWPPtXMnNJh3ERdf1o&q=${text}`
// //   const giphy = await getJson2(url)
// //   return giphy
// }



// fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
//     let body = req.body();
//     println!(body)
//     // ...
// }