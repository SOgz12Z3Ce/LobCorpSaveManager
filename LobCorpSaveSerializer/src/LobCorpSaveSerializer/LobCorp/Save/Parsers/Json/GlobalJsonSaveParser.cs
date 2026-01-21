using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class GlobalJsonSaveParser : JsonSaveParserBase
	{
		private static readonly List<string> regions = new List<string>()
		{
			"stat",
			"defense",
			"work_r",
			"work_w",
			"work_b",
			"work_p",
			"care_0",
			"care_1",
			"care_2",
			"care_3",
			"care_4",
			"care_5",
			"care_6",
		};
		public GlobalJsonSaveParser() : base(SaveType.Global)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var result = new Dictionary<string, object>();

			result["observe"] = ParseCodex(save["observe"] as JObject);
			result["etcData"] = ParseTracker(save["etcData"] as JObject);
			result["inventory"] = ParseEgo(save["inventory"] as JObject);
			result["research"] = ParseResearch(save["research"] as JObject);
			result["missions"] = ParseMission(save["missions"] as JObject);
			result["sefiraCharactes"] = ParseSephirah(save["sefiraCharactes"] as JObject);

			return result;
		}
		private static Dictionary<string, object> ParseCodex(JObject observe)
		{
			var inner = new Dictionary<long, Dictionary<string, object>>();
			var result = new Dictionary<string, object>()
			{
				{ "observeList", inner },
			};

			foreach (var kvp in observe["observeList"] as JObject)
			{
				inner[long.Parse(kvp.Key)] = ParseCodexEntry(kvp.Value as JObject);
			}

			return result;
		}
		private static Dictionary<string, object> ParseCodexEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<int>(entry, "observeProgress");
			result.CopyValue<int>(entry, "cubeNum");
			result.CopyValue<int>(entry, "totalKitUseCount");
			result.CopyValue<float>(entry, "totalKitUseTime");
			foreach (var region in regions)
			{
				result.TryCopyValue<bool>(entry, region);
			}

			return result;
		}
		private static Dictionary<string, object> ParseTracker(JObject etcData)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<int>(etcData, "day1clearCount");
			result.CopyValue<bool>(etcData, "tutorialDone");
			result.CopyValue<long>(etcData, "nextUnitInstanceId");
			result.CopyValue<int>(etcData, "unlockedMaxDay");
			result.CopyValue<bool>(etcData, "ending1Done");
			result.CopyValue<bool>(etcData, "ending2Done");
			result.CopyValue<bool>(etcData, "ending3Done");
			result.CopyValue<bool>(etcData, "trueEndingDone");
			result.CopyValue<bool>(etcData, "hiddenEndingDone");

			return result;
		}
		private static Dictionary<string, object> ParseEgo(JObject inventory)
		{
			var result = new Dictionary<string, object>();

			result["equips"] = ParseEgoList(inventory["equips"] as JArray);
			result.CopyValue<long>(inventory, "nextInstanceId");

			return result;
		}
		private static List<InventoryModel.EquipmentSaveData> ParseEgoList(JArray equips)
		{
			var result = new List<InventoryModel.EquipmentSaveData>();

			foreach (var ego in equips)
			{
				result.Add(new InventoryModel.EquipmentSaveData()
				{
					equipTypeId = ego["equipTypeId"].Value<int>(),
					equipInstanceId = ego["equipInstanceId"].Value<long>(),
				});
			}

			return result;
		}
		private static Dictionary<string, object> ParseResearch(JObject research)
		{
			var inner = new List<Dictionary<string, object>>();
			var result = new Dictionary<string, object>()
			{
				{ "research", inner },
			};

			foreach (var entry in research["research"])
			{
				inner.Add(ParseResearchEntry(entry as JObject));
			}

			return result;
		}
		private static Dictionary<string, object> ParseResearchEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<int>(entry, "researchItemTypeId");
			result.CopyValue<int>(entry, "curLevel");

			return result;
		}
		private static Dictionary<string, object> ParseMission(JObject missons)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<string>(missons, "ver");

			var inProgress = new List<Dictionary<string, object>>();
			foreach (var entry in missons["missionsInProgress"])
			{
				inProgress.Add(ParseMissionEntry(entry as JObject));
			}
			result["missionsInProgress"] = inProgress;

			var cleared = new List<Dictionary<string, object>>();
			foreach (var entry in missons["clearedMissions"])
			{
				cleared.Add(ParseMissionEntry(entry as JObject));
			}
			result["clearedMissions"] = cleared;

			var closed = new List<Dictionary<string, object>>();
			foreach (var entry in missons["closedMissions"])
			{
				closed.Add(ParseMissionEntry(entry as JObject));
			}
			result["closedMissions"] = closed;

			return result;
		}
		private static Dictionary<string, object> ParseMissionEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<int>(entry, "metadataId");

			return result;
		}
		private static Dictionary<string, object> ParseSephirah(JObject sefiraCharactes)
		{
			var inner = new Dictionary<string, object>();
			var result = new Dictionary<string, object>()
			{
				{ "sefiraChars", inner },
			};

			foreach (var entry in sefiraCharactes["sefiraChars"] as JObject)
			{
				inner[entry.Key] = ParseSephirahEntry(entry.Value as JObject);
			}

			return result;
		}
		private static Dictionary<string, object> ParseSephirahEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();

			result.CopyValue<int>(entry, "level");

			return result;
		}
	}
}
