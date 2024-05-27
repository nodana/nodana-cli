import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import promptly from "promptly";

import init from "../../src/commands/init";
import * as client from "../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

describe("commands/init", () => {
  let promptlyStub: SinonStub;
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    promptlyStub = sinon.stub(promptly, "confirm");
    promptlyStub.resolves(true);

    clientStub = sinon.stub(client, "init");
    clientStub.resolves({ key: "key-abc" });

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client init function with correct params", async () => {
    await init({});

    expect(clientStub).to.be.calledWith();
    expect(consoleStub).to.be.called;
  });
});
