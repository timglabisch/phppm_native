struct Client {

    pub fn send(data : T) -> Response
    where T : Into<Json> {

    }



}

struct ChannelCreateBuilder {
    pub name : &str,
    pub topic : Option<&str>
}

impl ChannelCreateBuilder {
    pub fn new(name : &str) -> Self {
        ChannelCreateBuilder {
            name : name,
            topic : None
        }
    }

    pub fn with_topic(&mut self, topic : &str) -> Self {
        self.topic = topic;
        self
    }
}


let foo = ChannelCreateBuilder::new("fooo");

impl ChannelCreateBuilder {

}

struct ChannelBuilder;

impl ChannelBuilder {
    pub fn create() -> ChannelCreateBuilder {
        ChannelCreateBuilder {}
    }
}

let channel_config = ChannelConfig::


let c = client::new();
let res = c.send(ChannelBuilder {
    foo: bar,
    ..ChannelBuilder::Default()
});
let res = c.send(ChannelBuilder::enable("foo"));
let res = c.send("channel?foo=bar");
let res = c.send(vec![("foo", "bar")]);
