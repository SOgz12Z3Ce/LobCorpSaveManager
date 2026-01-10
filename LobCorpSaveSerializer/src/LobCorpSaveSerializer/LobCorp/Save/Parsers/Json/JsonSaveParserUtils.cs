using System;
using System.Collections.Generic;
using Newtonsoft.Json.Linq;
using WorkerSprite;

namespace LobCorp.Save.Parsers.Json
{
	public static class JsonSaveParserUtils
	{
		public static WorkerSpriteSaveData.Pair ParseStyle(JObject style)
		{
			return new WorkerSpriteSaveData.Pair
			{
				a = style["a"].Value<long>(),
				b = style["b"].Value<int>(),
			};
		}
		public static WorkerSpriteSaveData.ColorData ParseColor(JObject color)
		{
			return new WorkerSpriteSaveData.ColorData
			{
				r = color["r"].Value<float>(),
				g = color["g"].Value<float>(),
				b = color["b"].Value<float>(),
				a = color["a"].Value<float>(),
			};
		}
		public static Dictionary<string, object> ParseDay(JObject day)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<string>(day, "saveVer");
			result.CopyValue<int>(day, "day");
			result["money"] = ParseLobPoint(day["money"] as JObject);
			result["agents"] = ParseAgent(day["agents"] as JObject);
			result["creatures"] = ParseAbnormality(day["creatures"] as JObject);
			result["playerData"] = ParseDate(day["playerData"] as JObject);
			result["sefiras"] = ParseSefirah(day["sefiras"] as JObject);
			result.CopyValue<string>(day, "saveState");
			result["agentName"] = new Dictionary<string, object>();
			return result;
		}
		private static Dictionary<string, object> ParseLobPoint(JObject money)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<int>(money, "money");
			return result;
		}
		private static Dictionary<string, object> ParseAgent(JObject agents)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<long>(agents, "nextInstId");
			var agentList = new List<Dictionary<string, object>>();
			foreach (var entry in agents["agentList"])
			{
				agentList.Add(ParseAgentEntry(entry as JObject));
			}
			result["agentList"] = agentList;
			return result;
		}
		private static Dictionary<string, object> ParseAgentEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<long>(entry, "instanceId");
			result.CopyValue<string>(entry, "currentSefira");
			result.CopyValue<string>(entry, "name");
			result.CopyValue<float>(entry, "baseMovement");
			result.CopyValue<int>(entry, "baseMaxHp");
			result.CopyValue<int>(entry, "baseMaxMental");
			result.CopyValue<string>(entry, "sefira");
			result["history"] = ParseAgentEntryStatistic(entry["history"] as JObject);
			result.CopyValue<bool>(entry, "iscustom");
			result.TryCopyValue<string>(entry, "customName");
			result.CopyValue<int>(entry, "nameId");
			result.CopyValue<bool>(entry, "isUniqueCredit");
			result.CopyValue<int>(entry, "uniqueScriptIndex");
			result["spriteSet"] = ParseAgentEntrySprite(entry["spriteSet"] as JObject);
			result["primaryStat"] = ParseAgentEntryVirtue(entry["primaryStat"] as JObject);
			result.CopyValue<int>(entry, "perfix");
			result.CopyValue<int>(entry, "suffix");
			result.CopyValue<long>(entry, "weaponId");
			result.CopyValue<long>(entry, "armorId");
			result["gifts"] = ParseAgentEntryGift(entry["gifts"] as JObject);
			result.CopyValue<string>(entry, "lastServiceSefira");
			result.CopyValue<int>(entry, "continuousServiceDay");
			result.CopyValue<bool>(entry, "isAce");
			return result;
		}
		private static Dictionary<string, object> ParseAgentEntryStatistic(JObject history)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<int>(history, "historyworkDay");
			result.CopyValue<int>(history, "workSuccess");
			result.CopyValue<int>(history, "workFail");
			result.CopyValue<int>(history, "physicalDamage");
			result.CopyValue<int>(history, "mentalDamage");
			result.CopyValue<int>(history, "deathByCreature");
			result.CopyValue<int>(history, "panicByCreature");
			result.CopyValue<int>(history, "deathByWorker");
			result.CopyValue<int>(history, "panic");
			result.CopyValue<int>(history, "creatureDamage");
			result.CopyValue<int>(history, "workerDamage");
			result.CopyValue<int>(history, "panicWorkerDamage");
			result.CopyValue<int>(history, "suppressDamage");
			result.CopyValue<int>(history, "disposition");
			result.CopyValue<int>(history, "promotionVal");
			result["workCubeCounts"] = ParseAgentEntryStatisticWorkCount(history["workCubeCounts"] as JObject);
			return result;
		}
		private static Dictionary<RwbpType, object> ParseAgentEntryStatisticWorkCount(JObject workCubeCounts)
		{
			return new Dictionary<RwbpType, object>()
			{
				{ RwbpType.R, workCubeCounts["R"].Value<int>() },
				{ RwbpType.W, workCubeCounts["W"].Value<int>() },
				{ RwbpType.B, workCubeCounts["B"].Value<int>() },
				{ RwbpType.P, workCubeCounts["P"].Value<int>() },
			};
		}
		private static Dictionary<string, object> ParseAgentEntrySprite(JObject spriteSet)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<bool>(spriteSet, "bInit");
			result["Eye"] = ParseStyle(spriteSet["Eye"] as JObject);
			result["EyeClose"] = ParseStyle(spriteSet["EyeClose"] as JObject);
			result["EyePanic"] = ParseStyle(spriteSet["EyePanic"] as JObject);
			result["EyeDead"] = ParseStyle(spriteSet["EyeDead"] as JObject);
			result["EyeBrow"] = ParseStyle(spriteSet["EyeBrow"] as JObject);
			result["BattleEyeBrow"] = ParseStyle(spriteSet["BattleEyeBrow"] as JObject);
			result["PanicEyeBrow"] = ParseStyle(spriteSet["PanicEyeBrow"] as JObject);
			result["EyeColor"] = ParseColor(spriteSet["EyeColor"] as JObject);
			result["Mouth"] = ParseStyle(spriteSet["Mouth"] as JObject);
			result["BattleMouth"] = ParseStyle(spriteSet["BattleMouth"] as JObject);
			result["PanicMouth"] = ParseStyle(spriteSet["PanicMouth"] as JObject);
			result["FrontHair"] = ParseStyle(spriteSet["FrontHair"] as JObject);
			result["RearHair"] = ParseStyle(spriteSet["RearHair"] as JObject);
			result["AttachmentHair"] = ParseStyle(spriteSet["AttachmentHair"] as JObject);
			result["HairColor"] = ParseColor(spriteSet["HairColor"] as JObject);
			return result;
		}
		private static WorkerPrimaryStat ParseAgentEntryVirtue(JObject primaryStat)
		{
			return new WorkerPrimaryStat()
			{
				hp = primaryStat["hp"].Value<int>(),
				mental = primaryStat["mental"].Value<int>(),
				work = primaryStat["work"].Value<int>(),
				battle = primaryStat["battle"].Value<int>(),
			};
		}
		private static Dictionary<string, object> ParseAgentEntryGift(JObject gifts)
		{
			var result = new Dictionary<string, object>();
			result.CopyValues<int>(gifts, "giftTypeIdList");
			result["lockState"] = ParseAgentEntryGiftLockState(gifts["lockState"] as JObject);
			result["displayState"] = ParseAgentEntryGiftDisplayState(gifts["displayState"] as JObject);
			return result;
		}
		private static Dictionary<int, object> ParseAgentEntryGiftLockState(JObject lockState)
		{
			var result = new Dictionary<int, object>();
			foreach (var entry in lockState)
			{
				result[int.Parse(entry.Key)] = ParseAgentEntryGiftLockStateEntry(entry.Value as JObject);
			}
			return result;
		}
		private static UnitEGOgiftSpace.GiftLockState ParseAgentEntryGiftLockStateEntry(JObject state)
		{
			return new UnitEGOgiftSpace.GiftLockState
			{
				id = state["id"].Value<long>(),
				state = state["state"].Value<bool>(),
			};
		}
		private static Dictionary<int, bool> ParseAgentEntryGiftDisplayState(JObject displayState)
		{
			var result = new Dictionary<int, bool>();
			foreach (var entry in displayState)
			{
				result[int.Parse(entry.Key)] = entry.Value.Value<bool>();
			}
			return result;
		}
		private static Dictionary<string, object> ParseAbnormality(JObject creatures)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<long>(creatures, "nextInstId");
			var creatureList = new List<Dictionary<string, object>>();
			foreach (var entry in creatures["creatureList"])
			{
				creatureList.Add(ParseAbnormalityEntry(entry as JObject));
			}
			return result;
		}
		private static Dictionary<string, object> ParseAbnormalityEntry(JObject entry)
		{
			var result = new Dictionary<string, object>();
			result.CopyValue<long>(entry, "instanceId");
			result.CopyValue<long>(entry, "metadataId");
			result.CopyValue<string>(entry, "entryNodeId");
			result.CopyValue<string>(entry, "sefiraNum");
			result["basePosition"] = new Vector2Serializer()
			{
				x = entry["x"].Value<float>(),
				y = entry["y"].Value<float>(),
			};
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
