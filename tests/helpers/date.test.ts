import chai from "chai";
import sinonChai from "sinon-chai";
import sinon, { SinonStub } from "sinon";
const expect = chai.expect;
chai.use(sinonChai);

import { getDurationString } from "../../src/helpers/date";

describe.only("date helper", () => {
  describe("When mins is not a number", () => {
    it("should return not available string", () => {
      // @ts-ignore
      const response = getDurationString("test");
      expect(response).to.equal("Duration not available");
    });
  });

  describe("When mins is less than 60", () => {
    it("should return minutes only", () => {
      const response = getDurationString(40);
      expect(response).to.equal("40 minutes");
    });
  });

  describe("When mins is greater than 60", () => {
    it("should return hours and minutes", () => {
      const response = getDurationString(140);
      expect(response).to.equal("2 hours, 20 minutes");
    });
  });
});