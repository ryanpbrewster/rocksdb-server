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
    client.put({ key: "my_key", value: `t = ${Date.now()}` }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting first get...");
  await new Promise(resolve => {
    client.get({ key: "my_key" }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting second get...");
  await new Promise(resolve => {
    client.get({ key: "non-existent key" }, (err, resp) => {
      if (err) {
        console.error(err);
      } else {
        console.log(resp);
      }
      resolve();
    });
  });

  console.log("starting full scan...");
  let scan = client.scan({});
  await new Promise(resolve => {
    scan.on('data', r => { console.log(r); });
    scan.on('end', () => { resolve(); });
    scan.on('error', e => { console.error(e); resolve(); });
  });
}

main();
