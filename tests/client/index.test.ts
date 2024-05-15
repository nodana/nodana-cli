import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";

const expect = chai.expect;
chai.use(sinonChai);

import { request } from "../../src/request";
import * as client from "../../src/client";

describe("client", () => {
  let requestStub: SinonStub;
  let apiKey = "12345.12345";
  let authHeader: any;

  beforeEach(() => {
    requestStub = sinon.stub(request, "_call");
    authHeader = client.getAuthorizationHeader(apiKey);
  });

  afterEach(() => {
    sinon.restore();
    authHeader = {};
  });

  describe("exchange", () => {
    it("should call request with correct arguments", async () => {
      const token = "12345";
      await client.exchange(token);

      expect(requestStub).to.have.been.calledWith(
        "/keys",
        "POST",
        {},
        { token }
      );
    });
  });

  describe("list", () => {
    it("should call request with correct arguments", async () => {
      await client.list(apiKey);

      expect(requestStub).to.have.been.calledWith(
        "/containers",
        "GET",
        authHeader
      );
    });
  });

  describe("start", () => {
    it("should call request with correct arguments", async () => {
      const options = {
        password: "12345",
      };
      await client.start(apiKey, options);

      expect(requestStub).to.have.been.calledWith(
        "/containers",
        "POST",
        authHeader,
        options
      );
    });
  });

  describe("stop", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "12345";
      await client.stop(apiKey, containerId);

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}`,
        "DELETE",
        authHeader
      );
    });
  });
});
