import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import status from "../../src/commands/status";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

describe("commands/status", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "status");
    clientStub.resolves({ key: "key-abc" });

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client status function with correct params", async () => {
    await status({});

    expect(clientStub).to.be.calledWith();
    expect(consoleStub).to.be.called;
  });
});
