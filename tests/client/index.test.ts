import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
import * as file from "../../src/helpers/file";

const expect = chai.expect;
chai.use(sinonChai);

import { request } from "../../src/request";
import * as client from "../../src/client";

describe("client", () => {
  let requestStub: SinonStub;
  let readFileStub: SinonStub;
  let writeFileStub: SinonStub;

  const key = "12345.12345";
  let authHeader: any;

  beforeEach(() => {
    requestStub = sinon.stub(request, "_call");
    readFileStub = sinon.stub(file, "read");
    writeFileStub = sinon.stub(file, "write");
    authHeader = client.getAuthHeader(key);
  });

  afterEach(() => {
    sinon.restore();
    authHeader = {};
  });

  describe("exchange", () => {
    beforeEach(() => {
      requestStub.resolves({ key: "12345" });
    });

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

    it("should write key in response to file", async () => {
      const token = "12345";
      await client.exchange(token);

      expect(writeFileStub).to.have.been.calledWith("12345");
    });
  });

  describe("create", () => {
    it("should call request with correct arguments", async () => {
      const options = {
        key,
        password: "12345",
      };
      await client.create(options);

      expect(requestStub).to.have.been.calledWith(
        "/containers",
        "POST",
        authHeader,
        options
      );
    });
  });

  describe("start", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "1";
      const options = {
        key,
      };
      await client.start(containerId, options);

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}/start`,
        "POST",
        authHeader,
        {}
      );
    });
  });

  describe("stop", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "1";
      const options = {
        key,
      };
      await client.stop(containerId, options);

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}/stop`,
        "POST",
        authHeader,
        {}
      );
    });
  });

  describe("list", () => {
    it("should call request with correct arguments", async () => {
      await client.list({ key });

      expect(requestStub).to.have.been.calledWith(
        "/containers",
        "GET",
        authHeader
      );
    });
  });

  describe("del", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "12345";
      await client.del(containerId, { key });

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}`,
        "DELETE",
        authHeader
      );
    });
  });
});