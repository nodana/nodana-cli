import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import start from "../../../src/commands/service/start";
import * as client from "../../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "1",
  status: "started",
};

describe("commands/service/start", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "startService");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client start function", async () => {
    const containerId = "container-1";
    await start(containerId);

    expect(clientStub).to.be.calledWith(containerId);
    expect(consoleStub).to.be.called;
  });
});
