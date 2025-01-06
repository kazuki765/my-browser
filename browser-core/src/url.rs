use alloc::string::String;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
  url: String,
  host: String,
  port: String,
  path: String,
  searchpart: String,
}

use alloc::string::ToString;
use alloc::vec::Vec;

impl Url {
  pub fn new(url: String) -> Self {
    Self {
      url,
      host: "".to_string(),
      port: "".to_string(),
      path: "".to_string(),
      searchpart: "".to_string(),
    }
  }
  
  pub fn host(&self) -> String {
    self.host.clone()
  }

  pub fn port(&self) -> String {
    self.port.clone()
  }

  pub fn path(&self) -> String {
    self.path.clone()
  }

  pub fn searchpart(&self) -> String {
    self.searchpart.clone() 
  }

  pub fn parse(&mut self) -> Result<Self, String> {
    if !self.is_http() {
      return Err("Only Http schema is supported".to_string());
    }
    
    self.host = self.extract_host();
    self.port = self.extract_port();
    self.path = self.extract_path();
    self.searchpart = self.extract_searchpart();

    Ok(self.clone())
  }
  fn is_http(&mut self) -> bool {
    self.url.contains("http://") 
  } 

  fn extract_host(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();

    // port番号が含まれているかどうかを判定
    if let Some(index) = url_parts[0].find(":") {
      url_parts[0][..index].to_string()
    } else {
      url_parts[0].to_string()
    }
  }

  fn extract_port(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();

    if let Some(index) = url_parts[0].find(":") {
      url_parts[0][index+1..].to_string()
    } else {
      "80".to_string()
    }
  } 

  fn extract_path(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();

    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

    path_and_searchpart[0].to_string()
  }

  fn extract_searchpart(&self) -> String {
    let url_parts: Vec<&str> = self.url.trim_start_matches("http://").splitn(2, "/").collect();

    if url_parts.len() < 2 {
      return "".to_string();
    }

    let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

    if path_and_searchpart.len() < 2 {
      "".to_string()
    } else  {
      path_and_searchpart[1].to_string()
    }
  } 
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_url_host() {
    // given
    let url = "http://example.com".to_string();

    // when
    let actual = Url::new(url).parse();

    // then
    let expected = Ok(
      Url {
        url: "http://example.com".to_string(),
        host: "example.com".to_string(),
        port: "80".to_string(), 
        path: "".to_string(),
        searchpart: "".to_string(),
      }
    );
    assert_eq!(expected, actual)

  }

  #[test]
  fn test_url_host_port(){
    // given
    let url = "http://example.com:8888".to_string();

    // when
    let actual = Url::new(url).parse();

    // then
    let expected = Ok(
      Url {
        url: "http://example.com:8888".to_string(),
        host: "example.com".to_string(),
        port: "8888".to_string(),
        path: "".to_string(),
        searchpart: "".to_string(),
      }
    );

    assert_eq!(expected, actual)
  }

  #[test]
  fn test_url_host_port_path() {
   // given
    let url = "http://example.com:8888/path".to_string();

    // when
    let actual = Url::new(url).parse();

    // then
    let expected = Ok(
      Url {
        url: "http://example.com:8888/path".to_string(),
        host: "example.com".to_string(),
        port: "8888".to_string(),
        path: "path".to_string(),
        searchpart: "".to_string(),

  });

    assert_eq!(expected, actual)
  }

  #[test]
  fn test_url_host_path(){
    let url = "http://example.com/path".to_string();

    let actual = Url::new(url).parse();

    let expected = Ok(
      Url {
        url: "http://example.com/path".to_string(),
        host: "example.com".to_string(),
        port: "80".to_string(),
        path: "path".to_string(),
        searchpart: "".to_string(),
      }
    );

    assert_eq!(expected, actual)
  }

  #[test]
  fn test_url_host_port_path_searchquery() {
    let url = "http://example.com:8888/index.html?a=123&b=456".to_string();

    let actual = Url::new(url).parse();

    let expected = Ok(
      Url {
        url: "http://example.com:8888/index.html?a=123&b=456".to_string(),
        host: "example.com".to_string(),
        port: "8888".to_string(),
        path: "index.html".to_string(),
        searchpart: "a=123&b=456".to_string(),
      }
    );

    assert_eq!(expected, actual);
  }

  #[test]
  fn test_no_scheme() {
    let url = "example.com".to_string();

    let actual = Url::new(url).parse();

    let expected = Err("Only Http schema is supported".to_string()); 
  
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_unsupported_scheme() {
    let url = "https://example.com".to_string();

    let actual = Url::new(url).parse();

    let expected = Err("Only Http schema is supported".to_string());

    assert_eq!(expected, actual);
  }
}