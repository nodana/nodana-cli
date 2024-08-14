import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";
import toml from "toml";
import * as file from "../../../src/helpers/file";

import create from "../../../src/commands/service/create";
import * as client from "../../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "12345",
  connectionUrl: "http://test.com",
};

describe("commands/service/create", () => {
  let promptlyStub: SinonStub;
  let clientStub: SinonStub;
  let consoleStub: SinonStub;
  let readFileStub: SinonStub;
  let tomlStub: SinonStub;

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    clientStub = sinon.stub(client, "createService");
    clientStub.resolves(mockResponse);

    readFileStub = sinon.stub(file, "readFile");
    readFileStub.resolves("");

    tomlStub = sinon.stub(toml, "parse");
    tomlStub.returns({ service: "phoenixd", settings: { name: "my-service" } });

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client create function with service and config params", async () => {
    await create({ config: "/file/path" });

    expect(clientStub).to.be.calledWith("phoenixd", { name: "my-service" });
    expect(consoleStub).to.be.called;
  });
});
