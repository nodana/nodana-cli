"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const chai_1 = __importDefault(require("chai"));
const sinon_chai_1 = __importDefault(require("sinon-chai"));
const sinon_1 = __importDefault(require("sinon"));
const expect = chai_1.default.expect;
chai_1.default.use(sinon_chai_1.default);
const request_1 = require("../../src/request");
const constants_1 = require("../../src/constants");
describe("request", () => {
    // const originalGlobalFetch = global.fetch;
    const fakeJson = sinon_1.default.fake.resolves({ foo: "bar" });
    const fakeResponse = { ok: true, status: 200, json: fakeJson };
    const fetchStub = sinon_1.default.stub();
    fetchStub.resolves(fakeResponse);
    beforeEach(() => {
        global.fetch = fetchStub;
    });
    afterEach(() => {
        // global.fetch = originalGlobalFetch;
        sinon_1.default.restore();
    });
    it("should make GET request with provided arguments", () => __awaiter(void 0, void 0, void 0, function* () {
        yield request_1.request.get("/path", {});
        expect(fetchStub).to.have.been.calledWith(`${constants_1.API_BASE_URL}/path`, {
            method: "GET",
            headers: { "Content-Type": "application/json" },
        });
    }));
    it("should make POST request with provided arguments", () => __awaiter(void 0, void 0, void 0, function* () {
        const testData = { foo: "bar" };
        yield request_1.request.post("/path", {}, testData);
        expect(fetchStub).to.have.been.calledWith(`${constants_1.API_BASE_URL}/path`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify(testData),
        });
    }));
    it("should make DELETE request with provided arguments", () => __awaiter(void 0, void 0, void 0, function* () {
        yield request_1.request.del("/path", {});
        expect(fetchStub).to.have.been.calledWith(`${constants_1.API_BASE_URL}/path`, {
            method: "DELETE",
            headers: { "Content-Type": "application/json" },
        });
    }));
    it("should include extra headers when provided", () => __awaiter(void 0, void 0, void 0, function* () {
        const extraHeaders = { Authorization: "Password" };
        yield request_1.request.get("/path", extraHeaders);
        expect(fetchStub).to.have.been.calledWith(`${constants_1.API_BASE_URL}/path`, {
            method: "GET",
            headers: Object.assign({ "Content-Type": "application/json" }, extraHeaders),
        });
    }));
    it("should return json data if response ok", () => __awaiter(void 0, void 0, void 0, function* () {
        const response = yield request_1.request.get("/path", {});
        expect(response.foo).to.eql("bar");
    }));
});
//# sourceMappingURL=index.test.js.map