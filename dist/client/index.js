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
exports.list = exports.stop = exports.start = exports.exchange = void 0;
const request_1 = __importDefault(require("../request"));
const getAuthorizationHeader = (key) => {
    const delimeter = ".";
    if (!key || !key.includes(delimeter))
        return {};
    const [username, password] = key.split(delimeter);
    return {
        Authorization: `Basic ${btoa(`${username}:${password}`)}`,
    };
};
const exchange = (...args_1) => __awaiter(void 0, [...args_1], void 0, function* (token = "") {
    try {
        return (0, request_1.default)("/keys", "POST", {}, { token });
    }
    catch (e) {
        throw e;
    }
});
exports.exchange = exchange;
const start = (key, options) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        return (0, request_1.default)("/containers", "POST", getAuthorizationHeader(key), options);
    }
    catch (e) {
        throw e;
    }
});
exports.start = start;
const stop = () => __awaiter(void 0, void 0, void 0, function* () { });
exports.stop = stop;
const list = (key) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        return (0, request_1.default)("/containers", "GET", getAuthorizationHeader(key));
    }
    catch (e) {
        throw e;
    }
});
exports.list = list;
//# sourceMappingURL=index.js.map