import chai from "chai";
import sinonChai from "sinon-chai";
const expect = chai.expect;
chai.use(sinonChai);

import { getDurationString } from "../../src/helpers/date";

describe("date helper", () => {
  describe("When mins is not a number", () => {
    it("should return not available string", () => {
      // @ts-ignore
      const response = getDurationString("test");
      expect(response).to.equal("Not available");
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

  describe("When mins is greater than 1 day", () => {
    it("should return days, hours and minutes", () => {
      const response = getDurationString(1500);
      expect(response).to.equal("1 days, 1 hours");
    });
  });
});
