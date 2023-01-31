pub trait ExchangeFunctionality {
    fn subscribe(&self);
    fn unsubscribe(&self);
    fn parse_data(&self);
}