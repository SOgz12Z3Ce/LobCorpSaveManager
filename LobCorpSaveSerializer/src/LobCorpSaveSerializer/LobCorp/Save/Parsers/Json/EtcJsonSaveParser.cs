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

			Copy<string>(data, save, "sefirabossTutorialPlayed");
			Copy<bool>(data, save, "e0");
			Copy<bool>(data, save, "e1");
			Copy<bool>(data, save, "e2");
			Copy<bool>(data, save, "e3");
			Copy<bool>(data, save, "e4");
			CopyList<long>(data, save, "waitingCreature");

			return data;
		}
	}
}
