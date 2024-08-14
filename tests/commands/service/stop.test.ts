import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import stop from "../../../src/commands/service/stop";
import * as date from "../../../src/helpers/date";
import * as client from "../../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "1",
  status: "stopped",
};

describe("commands/service/stop", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;
  let dateStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "stopService");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");

    dateStub = sinon.stub(date, "sleep");
    dateStub.resolves();
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client stop function", async () => {
    const containerId = "container-1";
    await stop(containerId);

    expect(clientStub).to.be.calledWith(containerId);
    expect(consoleStub).to.be.called;
  });
});
