import chai from "chai";
import sinonChai from "sinon-chai";
import sinon from "sinon";

const expect = chai.expect;
chai.use(sinonChai);

import request from "../../src/request";
import { API_BASE_URL } from "../../src/constants";

interface FakeResponse {
  readonly ok: boolean;
  readonly status: number;
  json(): Promise<any>;
}

describe("request", () => {
  const fakeJson = sinon.fake.resolves({ foo: "bar" });
  const fakeResponse: FakeResponse = { ok: true, status: 200, json: fakeJson };
  const fetchStub = sinon.stub();
  fetchStub.resolves(fakeResponse);

  beforeEach(() => {
    global.fetch = fetchStub;
    // const fetchStub = sinon.stub(global, "fetch");
    // fetchStub.returns(Promise.resolve(mockResponse));
  });

  afterEach(() => {
    // delete global.fetch
  });

  it("should make GET request with provided arguments", async () => {
    const response = await request("/path", "GET", {});

    expect(fetchStub).to.have.been.calledWith(`${API_BASE_URL}/path`, {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
  });
});
