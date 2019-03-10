const grpc = require("grpc");

const proto = grpc.load("../server/proto/helloworld/helloworld.proto");
const client = new proto.helloworld.Greeter("localhost:50051", grpc.credentials.createInsecure());

client.sayHello({ name: "Earl" }, (err, resp) => {
  if (err) {
    console.error(err);
  } else {
    console.log(resp);
  }
});
