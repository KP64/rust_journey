use core::marker::PhantomData;

#[derive(Debug, Default, Clone)]
pub enum HttpMethod {
    #[default]
    None,
    GET,
    HEAD,
    POST,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

#[derive(Debug, Clone)]
pub struct Request {
    pub url: String,
    pub method: HttpMethod,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}
impl Request {
    #[allow(clippy::new_ret_no_self)]
    #[must_use]
    pub fn new() -> RequestBuilder<NoUrl, NoMethod, NotSealed> {
        RequestBuilder::new()
    }
}

#[derive(Default, Debug, Clone)]
pub struct NotSealed;
#[derive(Default, Debug, Clone)]
pub struct Sealed;

#[derive(Default, Debug, Clone)]
pub struct NoUrl;
#[derive(Default, Debug, Clone)]
pub struct Url(String);

#[derive(Default, Debug, Clone)]
pub struct NoMethod;
#[derive(Default, Debug, Clone)]
pub struct Method(HttpMethod);

#[derive(Default, Debug, Clone)]
pub struct RequestBuilder<U, M, S> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
    sealed: PhantomData<S>,
}

impl RequestBuilder<NoUrl, NoMethod, NotSealed> {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

impl<S> RequestBuilder<Url, Method, S> {
    #[must_use]
    pub fn build(self) -> Request {
        Request {
            url: self.url.0,
            method: self.method.0,
            headers: self.headers,
            body: self.body,
        }
    }
}

impl<M> RequestBuilder<NoUrl, M, NotSealed> {
    pub fn url(self, url: impl Into<String>) -> RequestBuilder<Url, M, NotSealed> {
        RequestBuilder {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
            sealed: PhantomData,
        }
    }
}
impl<U> RequestBuilder<U, NoMethod, NotSealed> {
    pub fn method(self, method: HttpMethod) -> RequestBuilder<U, Method, NotSealed> {
        RequestBuilder {
            url: self.url,
            method: Method(method),
            headers: self.headers,
            body: self.body,
            sealed: PhantomData,
        }
    }
}

impl RequestBuilder<Url, Method, NotSealed> {
    #[must_use]
    pub fn seal(self) -> RequestBuilder<Url, Method, Sealed> {
        RequestBuilder {
            url: self.url,
            method: self.method,
            headers: self.headers,
            body: self.body,
            sealed: PhantomData,
        }
    }
}

impl<U, M> RequestBuilder<U, M, NotSealed> {
    #[must_use]
    pub fn body(mut self, body: impl Into<String>) -> Self {
        self.body = Some(body.into());
        self
    }
    #[must_use]
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }
}
