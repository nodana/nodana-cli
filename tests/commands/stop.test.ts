import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import stop from "../../src/commands/node/stop";
import * as date from "../../src/helpers/date";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "1",
  status: "stopped",
};

describe("commands/stop", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;
  let dateStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "stop");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");

    dateStub = sinon.stub(date, "sleep");
    dateStub.resolves();
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client stop function with option key if provided", async () => {
    const key = "key-1";
    const containerId = "container-1";
    await stop(containerId, { key });

    expect(clientStub).to.be.calledWith(containerId, { key });
    expect(consoleStub).to.be.called;
  });
});
