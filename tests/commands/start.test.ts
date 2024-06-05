import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import start from "../../src/commands/node/start";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "1",
  status: "started",
};

describe("commands/start", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "start");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client start function with option key if provided", async () => {
    const key = "key-1";
    const containerId = "container-1";
    await start(containerId, { key });

    expect(clientStub).to.be.calledWith(containerId, { key });
    expect(consoleStub).to.be.called;
  });
});
