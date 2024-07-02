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
  let writeFileStub: SinonStub;

  const key = "12345.12345";
  let authHeader: any;

  beforeEach(() => {
    requestStub = sinon.stub(request, "_call");
    writeFileStub = sinon.stub(file, "writeFile");
    authHeader = client.getAuthHeader(key);
  });

  afterEach(() => {
    sinon.restore();
    authHeader = {};
  });

  describe("init", () => {
    beforeEach(() => {
      requestStub.resolves({ key: "12345" });
    });

    it("should call request with correct arguments", async () => {
      await client.init();

      expect(requestStub).to.have.been.calledWith("/keys", "POST", {}, {});
    });

    it("should write key in response to file", async () => {
      await client.init();

      expect(writeFileStub).to.have.been.calledWith("12345");
    });
  });

  describe("status", () => {
    beforeEach(() => {
      requestStub.resolves({ key: "12345.abcde" });
    });

    it("should call request with correct arguments", async () => {
      const options = {
        key,
      };
      await client.status(options);

      expect(requestStub).to.have.been.calledWith(
        "/keys/12345",
        "GET",
        authHeader
      );
    });
  });

  describe("service/create", () => {
    it("should call request with correct arguments", async () => {
      const options = {
        key,
        password: "12345",
        type: "phoenixd",
      };
      await client.createService(options);

      expect(requestStub).to.have.been.calledWith(
        "/containers",
        "POST",
        authHeader,
        {
          config: {
            password: "12345",
            type: "phoenixd",
          },
        }
      );
    });
  });

  describe("service/start", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "1";
      const options = {
        key,
      };
      await client.startService(containerId, options);

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}/start`,
        "POST",
        authHeader,
        {}
      );
    });
  });

  describe("service/stop", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "1";
      const options = {
        key,
      };
      await client.stopService(containerId, options);

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}/stop`,
        "POST",
        authHeader,
        {}
      );
    });
  });

  describe("service/list", () => {
    it("should call request with correct arguments", async () => {
      await client.listServices({ key });

      expect(requestStub).to.have.been.calledWith(
        "/containers",
        "GET",
        authHeader
      );
    });
  });

  describe("service/delete", () => {
    it("should call request with correct arguments", async () => {
      const containerId = "12345";
      await client.deleteService(containerId, { key });

      expect(requestStub).to.have.been.calledWith(
        `/containers/${containerId}`,
        "DELETE",
        authHeader
      );
    });
  });

  describe("invoice/create", () => {
    it("should call request with correct arguments", async () => {
      const options = {
        key,
        value: 1000,
        memo: "Test memo",
      };
      await client.createInvoice(options);

      expect(requestStub).to.have.been.calledWith(
        "/invoices",
        "POST",
        authHeader,
        {
          value: 1000,
          memo: "Test memo",
        }
      );
    });
  });
});
