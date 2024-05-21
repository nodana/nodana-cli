import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";

import create from "../../src/commands/create";
import * as file from "../../src/helpers/file";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockContainerResponse = {
  id: "12345",
  connectionUrl: "http://test.com",
};

describe("commands/create", () => {
  let promptlyStub: SinonStub;
  let clientStub: SinonStub;
  let fileStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    clientStub = sinon.stub(client, "create");
    clientStub.resolves(mockContainerResponse);

    fileStub = sinon.stub(file, "read");

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should prioritise option key if one provided", async () => {
    const key = "12345";
    await create({ key });

    expect(clientStub).to.be.calledWith(key, {});
    expect(consoleStub).to.be.called;
  });

  it("should read conf file to get key if no key option provided", async () => {
    await create({});

    expect(fileStub).to.be.called;
  });
});
