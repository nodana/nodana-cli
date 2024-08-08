import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";

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

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    clientStub = sinon.stub(client, "createService");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client create function with option key if provided", async () => {
    const key = "12345";
    await create("phoenixd", { key });

    expect(clientStub).to.be.calledWith("phoenixd", { key });
    expect(consoleStub).to.be.called;
  });
});
