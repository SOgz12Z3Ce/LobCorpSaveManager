using System.Collections.Generic;
using LobCorp.Save.Type;
using Newtonsoft.Json.Linq;

namespace LobCorp.Save.Parsers.Json
{
	public class EtcJsonSaveParser : JsonSaveParserBase
	{
		public EtcJsonSaveParser() : base(SaveType.Etc)
		{
		}
		protected override Dictionary<string, object> Parse(JObject save)
		{
			var data = new Dictionary<string, object>();

			data.CopyValue<string>(save, "sefirabossTutorialPlayed");
			data.CopyValue<string>(save, "sefirabossTutorialPlayed");
			data.CopyValue<bool>(save, "e0");
			data.CopyValue<bool>(save, "e1");
			data.CopyValue<bool>(save, "e2");
			data.CopyValue<bool>(save, "e3");
			data.CopyValue<bool>(save, "e4");
			data.CopyValues<long>(save, "waitingCreature");

			return data;
		}
	}
}
