"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
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
const client = __importStar(require("../../src/client"));
describe("client", () => {
    let requestStub;
    let apiKey = "12345.12345";
    let authHeader;
    beforeEach(() => {
        requestStub = sinon_1.default.stub(request_1.request, "_call");
        authHeader = client.getAuthorizationHeader(apiKey);
    });
    afterEach(() => {
        sinon_1.default.restore();
        authHeader = {};
    });
    describe("exchange", () => {
        it("should call request with correct arguments", () => __awaiter(void 0, void 0, void 0, function* () {
            const token = "12345";
            yield client.exchange(token);
            expect(requestStub).to.have.been.calledWith("/keys", "POST", {}, { token });
        }));
    });
    describe("list", () => {
        it("should call request with correct arguments", () => __awaiter(void 0, void 0, void 0, function* () {
            yield client.list(apiKey);
            expect(requestStub).to.have.been.calledWith("/containers", "GET", authHeader);
        }));
    });
    describe("start", () => {
        it("should call request with correct arguments", () => __awaiter(void 0, void 0, void 0, function* () {
            const options = {
                password: "12345",
            };
            yield client.start(apiKey, options);
            expect(requestStub).to.have.been.calledWith("/containers", "POST", authHeader, options);
        }));
    });
    describe("stop", () => {
        it("should call request with correct arguments", () => __awaiter(void 0, void 0, void 0, function* () {
            const containerId = "12345";
            yield client.stop(apiKey, containerId);
            expect(requestStub).to.have.been.calledWith(`/containers/${containerId}`, "DELETE", authHeader);
        }));
    });
});
//# sourceMappingURL=index.test.js.map