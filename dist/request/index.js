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
exports.request = void 0;
const constants_1 = require("../constants");
exports.request = {
    get: function (path, headers) {
        return __awaiter(this, void 0, void 0, function* () {
            return this._call(path, "GET", headers);
        });
    },
    post: function (path, headers, body) {
        return __awaiter(this, void 0, void 0, function* () {
            return this._call(path, "POST", headers, body);
        });
    },
    del: function (path, headers) {
        return __awaiter(this, void 0, void 0, function* () {
            return this._call(path, "DELETE", headers);
        });
    },
    _call: (path, method, extraHeaders, data) => __awaiter(void 0, void 0, void 0, function* () {
        try {
            const options = {
                method,
                headers: Object.assign({ "Content-Type": "application/json" }, extraHeaders),
            };
            if (data) {
                options.body = JSON.stringify(data);
            }
            const response = yield fetch(`${constants_1.API_BASE_URL}${path}`, options);
            const json = yield response.json();
            if (!response.ok) {
                throw new Error(json.message);
            }
            return json;
        }
        catch (e) {
            throw e;
        }
    }),
};
//# sourceMappingURL=index.js.map