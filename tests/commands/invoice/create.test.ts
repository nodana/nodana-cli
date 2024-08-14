import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

import create from "../../../src/commands/invoice/create";
import * as client from "../../../src/client";

const expect = chai.expect;
chai.use(sinonChai);

const mockResponse = {
  id: "12345",
  hash: "dhjkahsdjkashdkhaskdjhas",
  pr: "lnc1shdhsakdhkashdkasd",
  value: 1000,
  memo: "test memo",
};

describe("commands/invoice/create", () => {
  let clientStub: SinonStub;
  let consoleStub: SinonStub;

  beforeEach(() => {
    clientStub = sinon.stub(client, "createInvoice");
    clientStub.resolves(mockResponse);

    consoleStub = sinon.stub(console, "log");
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should call client create function", async () => {
    const options = {
      sats: 1000,
    };

    await create(options);

    expect(clientStub).to.be.calledWith(options);
    expect(consoleStub).to.be.called;
  });
});
