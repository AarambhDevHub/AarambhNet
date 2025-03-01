// http.rs
use reqwest::{header::HeaderMap, Client, Response, Url};
use std::error::Error;
#[cfg(feature = "logger")]
use tracing::info;

/// The `HttpClient` struct in Rust represents an HTTP client with a base URL, optional default headers,
/// and a client instance.
/// 
/// # Properties:
/// 
/// * `base_url`: The `base_url` property in the `HttpClient` struct represents the base URL that will
/// be used for making HTTP requests. This URL serves as the starting point for constructing full URLs
/// for the requests sent by the HTTP client.
/// * `default_headers`: The `default_headers` property in the `HttpClient` struct is an optional field
/// that can hold a `HeaderMap`. This field can be used to store default headers that will be included
/// in every request made by the `HttpClient`. If no default headers are provided, this field will be
/// `None`.
/// * `client`: The `client` property in the `HttpClient` struct is of type `Client`. This likely
/// represents an HTTP client that can be used to make HTTP requests to a server. The `Client` type is
/// commonly used in Rust libraries like `reqwest` for sending HTTP requests and handling responses.
pub struct HttpClient {
    base_url: Url,
    default_headers: Option<HeaderMap>,
    client: Client
}

/// The `impl HttpClient { ... }` block in the Rust code snippet is implementing methods for the
/// `HttpClient` struct. Here's a breakdown of what each method is doing:
impl HttpClient {
    /// The function `new` creates a new instance of an `HttpClient` with a base URL, default headers,
    /// and a new client.
    /// 
    /// # Arguments:
    /// 
    /// * `base_url`: The `base_url` parameter is a string reference (`&str`) that represents the base
    /// URL for the HTTP client. This is the URL that will be used as the starting point for making HTTP
    /// requests.
    /// * `default_headers`: The `default_headers` parameter in the `new` function is an optional
    /// parameter of type `Option<HeaderMap>`. It allows you to provide a set of default headers to be
    /// included in each HTTP request made by the `HttpClient`. If no default headers are provided, you
    /// can pass `None
    /// 
    /// # Returns:
    /// 
    /// The `new` function is returning a `Result` containing an instance of `HttpClient` if the URL
    /// parsing is successful and the `HttpClient` struct is properly initialized with the provided base
    /// URL, default headers, and a new `Client` instance.
    pub fn new(base_url: &str, default_headers: Option<HeaderMap>) -> Result<Self, Box<dyn Error>> {
        #[cfg(feature = "logger")]
        info!("Initializing HttpClient with base URL: {}", base_url);
        Ok(HttpClient {
            base_url: Url::parse(base_url)?,
            default_headers,
            client: Client::new(),
        })
    }

    /// The function `merge_headers` merges default headers with any extra headers provided and returns
    /// the resulting `HeaderMap`.
    /// 
    /// # Arguments:
    /// 
    /// * `headers`: Option<HeaderMap>
    /// 
    /// # Returns:
    /// 
    /// The `merge_headers` function returns a `HeaderMap` which contains the merged headers from
    /// `self.default_headers` and the `headers` provided as an argument.
    fn merge_headers(&self, headers: Option<HeaderMap>) -> HeaderMap {
        let mut merged_headers = self.default_headers.clone().unwrap_or_else(HeaderMap::new);
        if let Some(extra_headers) = headers {
            for (key, value) in extra_headers.iter() {
                merged_headers.insert(key.clone(), value.clone());
            }
        }
        #[cfg(feature = "logger")]
        info!("Merged headers: {:?}", merged_headers);
        merged_headers
    }

    /// This Rust function performs an asynchronous HTTP GET request with specified headers.
    /// 
    /// # Arguments:
    /// 
    /// * `endpoint`: The `endpoint` parameter in the `get` function represents the specific endpoint or
    /// path that you want to access on the base URL. It is a string that typically corresponds to a
    /// specific resource or action on the server.
    /// * `headers`: The `headers` parameter in the `get` function is an optional `HeaderMap` type. It
    /// allows you to pass additional headers that will be merged with the default headers before making
    /// the HTTP request. If no additional headers are needed, you can pass `None` as the value for this
    /// parameter
    /// 
    /// # Returns:
    /// 
    /// The `get` function returns a `Result` containing a `Response` if the request is successful, or a
    /// `Box<dyn Error>` if an error occurs during the request.
    pub async fn get(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, Box<dyn Error>> {
        let url = self.base_url.join(endpoint)?;
        #[cfg(feature = "logger")]
        info!("Sending GET request to {}", url);
        let merged_headers = self.merge_headers(headers);
        let response = self.client.get(url).headers(merged_headers).send().await?;
        Ok(response)
    }

    /// The function `post` sends an asynchronous POST request with optional headers and body, returning
    /// a Result containing the response.
    /// 
    /// # Arguments:
    /// 
    /// * `endpoint`: The `endpoint` parameter in the `post` function represents the specific endpoint
    /// or route that you want to send a POST request to. It is a string that typically comes after the
    /// base URL of the API you are interacting with.
    /// * `headers`: The `headers` parameter in the `post` function is an optional `HeaderMap` type. It
    /// allows you to pass additional headers to be included in the HTTP request. If you don't need to
    /// include any extra headers, you can pass `None` as the value for this parameter. If
    /// * `body`: The `body` parameter in the `post` function represents the payload or data that you
    /// want to send in the HTTP request body. It is an optional parameter, meaning you can choose to
    /// include a body or not when making a POST request. If you provide a body, it should be a string
    /// 
    /// # Returns:
    /// 
    /// The `post` function returns a `Result` containing a `Response` if the operation is successful,
    /// or a `Box` containing a dynamic error trait object if an error occurs.
    pub async fn post(&self, endpoint: &str, headers: Option<HeaderMap>, body: Option<&str>) -> Result<Response, Box<dyn Error>> {
        let url = self.base_url.join(endpoint)?;
        #[cfg(feature = "logger")]
        info!("Sending POST request to {}", url);
        let merged_headers = self.merge_headers(headers);
        let mut request = self.client.post(url).headers(merged_headers);

        // If a body is provided, add it to the request
        if let Some(b) = body {
            request = request.body(b.to_string());
        }

        let response = request.send().await?;
        Ok(response)
    }

    /// The function `put` sends an HTTP PUT request with optional headers and body, and returns the
    /// response asynchronously.
    /// 
    /// # Arguments:
    /// 
    /// * `endpoint`: The `endpoint` parameter is a string that represents the specific endpoint or
    /// route that you want to send a PUT request to. It is typically a part of the URL path after the
    /// base URL.
    /// * `headers`: The `headers` parameter is an optional `HeaderMap` type, which represents a
    /// collection of HTTP headers. It allows you to pass additional headers along with the request. If
    /// no headers are needed, you can pass `None` as the value for this parameter.
    /// * `body`: The `body` parameter in the `put` function is an optional reference to a string. It
    /// represents the body content that will be sent in the HTTP PUT request. If a value is provided
    /// for the `body` parameter, it will be included in the request; otherwise, the request will be
    /// 
    /// # Returns:
    /// 
    /// The `put` function returns a `Result` containing a `Response` if the operation is successful, or
    /// a `Box` containing a trait object that implements the `Error` trait if an error occurs.
    pub async fn put(&self, endpoint: &str, headers: Option<HeaderMap>, body: Option<&str>) -> Result<Response, Box<dyn Error>> {
        let url = self.base_url.join(endpoint)?;
        #[cfg(feature = "logger")]
        info!("Sending PUT request to {}", url);
        let merged_headers = self.merge_headers(headers);
        let mut request = self.client.put(url).headers(merged_headers);

        if let Some(b) = body {
            request = request.body(b.to_string());
        }

        let response = request.send().await?;
        Ok(response)
    }

    /// The function `delete` sends a DELETE request to a specified endpoint with optional headers and
    /// returns the response asynchronously.
    /// 
    /// # Arguments:
    /// 
    /// * `endpoint`: The `endpoint` parameter in the `delete` function is a reference to a string that
    /// represents the specific endpoint or resource path that you want to delete on the server. It is
    /// used to construct the complete URL for the DELETE request.
    /// * `headers`: The `headers` parameter in the `delete` function is an optional `HeaderMap` type.
    /// It allows you to pass additional headers to be included in the HTTP request. If no headers are
    /// needed, you can pass `None` as the value for this parameter. If you do need to include
    /// 
    /// # Returns:
    /// 
    /// The `delete` function returns a `Result` containing a `Response` if the operation is successful,
    /// or a `Box<dyn Error>` if an error occurs.
    pub async fn delete(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, Box<dyn Error>> {
        let url = self.base_url.join(endpoint)?;
        #[cfg(feature = "logger")]
        info!("Sending DELETE request to {}", url);
        let merged_headers = self.merge_headers(headers);
        let response = self.client.delete(url).headers(merged_headers).send().await?;
        Ok(response)
    }

    /// This Rust function sends a HEAD request to a specified endpoint with optional headers and
    /// returns the response asynchronously.
    /// 
    /// # Arguments:
    /// 
    /// * `endpoint`: The `endpoint` parameter in the `head` function represents the specific path or
    /// resource on the server that you want to send a HTTP HEAD request to. It is typically a string
    /// that specifies the endpoint URL relative to the base URL of the API.
    /// * `headers`: The `headers` parameter in the `head` function is an optional `HeaderMap` type. It
    /// allows you to pass additional headers to be included in the HTTP request. If you don't need to
    /// include any extra headers, you can pass `None` as the value for this parameter. If
    /// 
    /// # Returns:
    /// 
    /// The `head` function returns a `Result` containing a `Response` if the operation is successful,
    /// or a `Box<dyn Error>` if an error occurs.
    pub async fn head(&self, endpoint: &str, headers: Option<HeaderMap>) -> Result<Response, Box<dyn Error>> {
        let url = self.base_url.join(endpoint)?;
        #[cfg(feature = "logger")]
        info!("Sending HEAD request to {}", url);
        let merged_headers = self.merge_headers(headers);
        let response = self.client.head(url).headers(merged_headers).send().await?;
        Ok(response)
    }

    /// The function `patch` sends a PATCH request to a specified endpoint with optional headers and body,
    /// and returns the response asynchronously.
    /// 
    /// # Arguments:
    /// 
    /// * `endpoint`: The `endpoint` parameter in the `patch` function is a string that represents the
    /// specific endpoint or route that you want to send a PATCH request to. It is typically a part of the
    /// URL after the base URL.
    /// * `headers`: The `headers` parameter in the `patch` function is an optional `HeaderMap` type. It
    /// allows you to pass additional headers to be included in the HTTP request. If you don't need to
    /// include any extra headers, you can pass `None` as the value for this parameter. If
    /// * `body`: The `body` parameter in the `patch` function is an optional reference to a string
    /// (`Option<&str>`). It represents the body content that will be sent in the HTTP request when making a
    /// PATCH request to the specified `endpoint`. If a value is provided for the `body`, it will
    /// 
    /// # Returns:
    /// 
    /// The `patch` function returns a `Result` containing either a `Response` or a boxed trait object
    /// implementing the `Error` trait.
    pub async fn patch(&self, endpoint: &str, headers: Option<HeaderMap>, body: Option<&str>) -> Result<Response, Box<dyn Error>> {
        let url = self.base_url.join(endpoint)?;
        #[cfg(feature = "logger")]
        info!("Sending PATCH request to {}", url);
        let merged_headers = self.merge_headers(headers);
        let mut request = self.client.patch(url).headers(merged_headers);

        if let Some(b) = body {
            request = request.body(b.to_string());
        }

        let response = request.send().await?;
        Ok(response)
    }

}


#[cfg(test)]
mod test {
    use super::*;

    fn setup_client() -> HttpClient {
        let base_url = "https://httpbin.org";
        HttpClient::new(base_url, None).unwrap()
    }

    #[tokio::test]
    async fn test_get_request() {
        let client = setup_client();
        let endpoint = "/get";

        match client.get(endpoint, None).await {
            Ok(response) => {
                assert_eq!(response.status(), 200); // Assert that the status is 200 OK
                let body = response.text().await.unwrap();
                assert!(body.contains("\"url\": \"https://httpbin.org/get\"")); // Assert the response contains the expected URL
                println!("GET Response: {}", body);
            },
            Err(e) => panic!("GET request failed: {}", e),
        }
    }

}