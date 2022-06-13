pub trait Request {
	fn path(&self) -> &str;

	fn query_string(&self) -> &str;
}