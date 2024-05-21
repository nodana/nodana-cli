import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import stop from "../../src/commands/stop";
import * as file from "../../src/helpers/file";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "1",
  status: "stopped",
};

describe("commands/stop", () => {
  let clientStub: SinonStub;
  let fileStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    fileStub = sinon.stub(file, "read");
    fileStub.resolves("file-key");

    clientStub = sinon.stub(client, "stop");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client stop function with option key if provided", async () => {
    const key = "key-1";
    const containerId = "container-1";
    await stop(containerId, { key });

    expect(clientStub).to.be.calledWith(key, containerId);
    expect(consoleStub).to.be.called;
  });

  it("should read conf file if no option key provided", async () => {
    const containerId = "container-1";
    await stop(containerId, {});

    expect(fileStub).to.be.called;
    expect(clientStub).to.be.calledWith("file-key", containerId);
    expect(consoleStub).to.be.called;
  });
});
