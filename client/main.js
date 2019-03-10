const grpc = require("grpc");
const loader = require("@grpc/proto-loader");

const definition = loader.loadSync("../proto/kvstore.proto");
const proto = grpc.loadPackageDefinition(definition);
const client = new proto.kvstore.KvStore("localhost:50051", grpc.credentials.createInsecure());

async function main() {
  console.log("starting sayHello...");
  await new Promise(resolve => {
    client.sayHello({ name: "Earl" }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting put...");
  await new Promise(resolve => {
    client.put({ name: "Earl" }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting get...");
  await new Promise(resolve => {
    client.get({ name: "Earl" }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });
}

main();
