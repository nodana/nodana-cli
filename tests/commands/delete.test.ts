import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";

import del from "../../src/commands/delete";
import * as file from "../../src/helpers/file";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "1",
  status: "deleted",
};
describe("commands/delete", () => {
  let promptlyStub: SinonStub;
  let clientStub: SinonStub;
  let fileStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    fileStub = sinon.stub(file, "read");
    fileStub.resolves("file-key");

    clientStub = sinon.stub(client, "del");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client del function with option key if provided", async () => {
    const key = "key-1";
    const containerId = "container-1";
    await del(containerId, { key });

    expect(clientStub).to.be.calledWith(key, containerId);
    expect(consoleStub).to.be.called;
  });

  it("should read conf file if no option key provided", async () => {
    const containerId = "container-1";
    await del(containerId, {});

    expect(fileStub).to.be.called;
    expect(clientStub).to.be.calledWith("file-key", containerId);
    expect(consoleStub).to.be.called;
  });
});
