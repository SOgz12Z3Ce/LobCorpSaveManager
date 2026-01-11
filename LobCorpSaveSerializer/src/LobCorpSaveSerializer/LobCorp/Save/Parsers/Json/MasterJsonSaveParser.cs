using System.Collections.Generic;
using System.Globalization;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class MasterJsonSaveParser : JsonSaveParserBase
	{
		public MasterJsonSaveParser() : base(SaveType.Master)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<string>(save, "saveVer");
			result.CopyValue<float>(save, "playTime");
			result.CopyValue<int>(save, "lastDay");
			result.CopyValue<int>(save, "checkPointDay");
			var dayList = new Dictionary<int, Dictionary<string, object>>();
			foreach (var day in save["dayList"] as JObject)
			{
				dayList[int.Parse(day.Key)] = ParseDay(day.Value as JObject);
			}
			result["dayList"] = dayList;

			return result;
		}
		private static Dictionary<string, object> ParseDay(JObject day)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<string>(day, "saveInnerVer");
			result.CopyValue<int>(day, "day");
			result["money"] = JsonSaveParserUtils.ParseLobPoint(day["money"] as JObject);
			result["agents"] = JsonSaveParserUtils.ParseAgent(day["agents"] as JObject);
			result["creatures"] = JsonSaveParserUtils.ParseAbnormality(day["creatures"] as JObject);
			result["playerData"] = ParseDate(day["playerData"] as JObject);
			result["sefiras"] = ParseSefirah(day["sefiras"] as JObject);
			result.CopyValue<string>(day, "saveState");
			result["agentName"] = new Dictionary<string, object>();
			return result;
		}
		private static Dictionary<string, object> ParseDate(JObject playerData)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<int>(playerData, "day");
			return result;
		}
		private static Dictionary<string, object> ParseSefirah(JObject sefiras)
		{
			var result = new Dictionary<string, object>();
			foreach (var entry in sefiras)
			{
				result[entry.Key] = ParseSefirahEntry(entry.Value as JObject);
			}
			return result;
		}
		private static Dictionary<string, object> ParseSefirahEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<bool>(entry, "activated");
			result.CopyValue<int>(entry, "openLevel");
			return result;
		}
	}
}
