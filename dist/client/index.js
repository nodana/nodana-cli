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
Object.defineProperty(exports, "__esModule", { value: true });
exports.list = exports.stop = exports.start = exports.exchange = exports.getAuthorizationHeader = void 0;
const request_1 = require("../request");
const getAuthorizationHeader = (key) => {
    const delimeter = ".";
    if (!key || !key.includes(delimeter))
        return {};
    const [username, password] = key.split(delimeter);
    return {
        Authorization: `Basic ${btoa(`${username}:${password}`)}`,
    };
};
exports.getAuthorizationHeader = getAuthorizationHeader;
const exchange = (...args_1) => __awaiter(void 0, [...args_1], void 0, function* (token = "") {
    try {
        return request_1.request._call("/keys", "POST", {}, { token });
    }
    catch (e) {
        throw e;
    }
});
exports.exchange = exchange;
const start = (key, options) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        return request_1.request._call("/containers", "POST", (0, exports.getAuthorizationHeader)(key), options);
    }
    catch (e) {
        throw e;
    }
});
exports.start = start;
const stop = (key, id) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        return request_1.request._call(`/containers/${id}`, "DELETE", (0, exports.getAuthorizationHeader)(key));
    }
    catch (e) {
        throw e;
    }
});
exports.stop = stop;
const list = (key) => __awaiter(void 0, void 0, void 0, function* () {
    try {
        return request_1.request._call("/containers", "GET", (0, exports.getAuthorizationHeader)(key));
    }
    catch (e) {
        throw e;
    }
});
exports.list = list;
//# sourceMappingURL=index.js.map