import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import exchange from "../../src/commands/exchange";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

describe("commands/exchange", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "exchange");
    clientStub.resolves({ key: "key-abc" });

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client exchange function with correct params", async () => {
    const token = "12345";
    await exchange({ token });

    expect(clientStub).to.be.calledWith(token);
    expect(consoleStub).to.be.called;
  });
});
