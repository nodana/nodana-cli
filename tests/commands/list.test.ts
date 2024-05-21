import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";

import list from "../../src/commands/list";
import * as file from "../../src/helpers/file";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = [
  {
    id: "12345",
    connectionUrl: "http://test.com",
  },
];

describe("commands/list", () => {
  let promptlyStub: SinonStub;
  let clientStub: SinonStub;
  let fileStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    clientStub = sinon.stub(client, "list");
    clientStub.resolves(mockResponse);

    fileStub = sinon.stub(file, "read");
    fileStub.resolves("file-key");

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client list function with option key if provided", async () => {
    const key = "12345";
    await list({ key });

    expect(clientStub).to.be.calledWith(key);
    expect(consoleStub).to.be.called;
  });

  it("should read conf file if no option key provided", async () => {
    await list({});

    expect(fileStub).to.be.called;
    expect(clientStub).to.be.calledWith("file-key");
    expect(consoleStub).to.be.called;
  });
});
