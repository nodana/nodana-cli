import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";

import list from "../../../src/commands/node/list";
import * as client from "../../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = [
  {
    id: "12345",
    connectionUrl: "http://test.com",
  },
];

describe("commands/node/list", () => {
  let promptlyStub: SinonStub;
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    clientStub = sinon.stub(client, "listNodes");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client list function with option key if provided", async () => {
    const key = "12345";
    await list({ key });

    expect(clientStub).to.be.calledWith({ key });
    expect(consoleStub).to.be.called;
  });
});
