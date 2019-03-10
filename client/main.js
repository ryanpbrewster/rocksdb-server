const grpc = require("grpc");
const loader = require("@grpc/proto-loader");

const definition = loader.loadSync("../server/proto/helloworld/helloworld.proto");
const proto = grpc.loadPackageDefinition(definition);
const client = new proto.helloworld.Greeter("localhost:50051", grpc.credentials.createInsecure());

client.sayHello({ name: "Earl" }, (err, resp) => {
  if (err) {
    console.error(err);
  } else {
    console.log(resp);
  }
});
