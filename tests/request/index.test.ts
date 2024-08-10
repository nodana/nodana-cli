import chai from "chai";
import sinonChai from "sinon-chai";
import sinon from "sinon";

const expect = chai.expect;
chai.use(sinonChai);

import { request } from "../../src/request";

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
  });

  afterEach(() => {
    sinon.restore();
  });

  it("should make GET request with provided arguments", async () => {
    await request.get("/path", {});

    expect(fetchStub).to.have.been.calledWith("https://api.nodana.io/v1/path", {
      method: "GET",
      headers: { "Content-Type": "application/json" },
    });
  });

  it("should make POST request with provided arguments", async () => {
    const testData = { foo: "bar" };
    await request.post("/path", {}, testData);

    expect(fetchStub).to.have.been.calledWith("https://api.nodana.io/v1/path", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify(testData),
    });
  });

  it("should make DELETE request with provided arguments", async () => {
    await request.del("/path", {});

    expect(fetchStub).to.have.been.calledWith("https://api.nodana.io/v1/path", {
      method: "DELETE",
      headers: { "Content-Type": "application/json" },
      body: "{}",
    });
  });

  it("should include extra headers when provided", async () => {
    const extraHeaders = { Authorization: "Password" };
    await request.get("/path", extraHeaders);

    expect(fetchStub).to.have.been.calledWith("https://api.nodana.io/v1/path", {
      method: "GET",
      headers: { "Content-Type": "application/json", ...extraHeaders },
    });
  });

  it("should return json data if response ok", async () => {
    const response = await request.get("/path", {});

    expect(response.foo).to.eql("bar");
  });
});
